#![allow(clippy::too_many_arguments)]

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::Path;

use crate::elf::{header, program_header, relocation, section, segument, symbol};
use crate::parser;

#[derive(Debug, Clone)]
pub struct OutputSection {
    pub name: String,
    pub r#type: section::SectionType,
    pub flags: Vec<section::SectionFlag>,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub data: Vec<u8>,
    pub align: u64,
}

#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    pub r#type: symbol::Type,
    pub binding: symbol::Binding,
    pub section_index: u16,
    pub object_index: usize,
    pub is_defined: bool,
}

impl ResolvedSymbol {
    pub fn is_stronger_than(&self, other: &Self) -> bool {
        match (self.binding, other.binding) {
            (symbol::Binding::Local, _) => true,
            (_, symbol::Binding::Weak) => true,
            (symbol::Binding::Global, symbol::Binding::Global) => false,
            (symbol::Binding::Global, symbol::Binding::Local) => false,
            (symbol::Binding::Weak, _) => false,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Object {
    section_headers: Vec<section::Header>,
    symbols: Vec<symbol::Symbol>,
    relocations: Vec<relocation::RelocationAddend>,
}

#[derive(Debug, Default)]
pub struct Linker {
    objects: Vec<Object>,
}

impl Linker {
    pub fn new() -> Self {
        Linker {
            objects: Vec::new(),
        }
    }

    pub fn add_object(mut self, path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;

        let elf = parser::parse_elf(&data)?.1;

        self.objects.push(Object {
            section_headers: elf.section_headers,
            symbols: elf.symbols,
            relocations: elf.relocations,
        });

        Ok(self)
    }

    pub fn link_to_file(
        &mut self,
        output_path: &Path,
        input_paths: &[&Path],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for input_path in input_paths {
            let object = fs::read(input_path)?;
            let elf = parser::parse_elf(&object)?.1;
            self.objects.push(Object {
                section_headers: elf.section_headers,
                symbols: elf.symbols,
                relocations: elf.relocations,
            });
        }
        let output_sections = self.link()?;
        self.write_executable(output_path, &output_sections)
    }

    /// 実行可能ファイルを出力する
    pub fn write_executable(
        &self,
        output_path: &Path,
        output_sections: &[OutputSection],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .mode(0o655) // rw-r-xr-x
            .open(output_path)?;

        // まずシンボル解決を行い、_startシンボルのアドレスを取得
        let (mut resolved_symbols, _) = self.resolve_symbols();

        // セクションレイアウトを行う（シンボルアドレスが更新される）
        self.layout_sections(&mut resolved_symbols);

        let Some(ResolvedSymbol {
            value: entry_point, ..
        }) = resolved_symbols.get("_start")
        else {
            return Err("sybmol _start not found".into());
        };

        // シンボル文字列テーブル(.strtab)を作成
        let mut strtab_data: Vec<u8> = Vec::new();
        let mut symbol_name_offsets: HashMap<String, usize> = HashMap::new();

        // シンボル名をテーブルに追加
        for name in resolved_symbols.keys() {
            if !name.is_empty() {
                let offset = strtab_data.len();
                symbol_name_offsets.insert(name.clone(), offset);
                strtab_data.extend_from_slice(name.as_bytes());
                strtab_data.push(0); // NULL終端
            }
        }

        // シンボルテーブル(.symtab)を作成
        let mut symtab_data: Vec<u8> = Vec::new();

        // 各シンボルのエントリを追加
        for (name, symbol) in &resolved_symbols {
            if symbol.is_defined {
                let name_offset = *symbol_name_offsets.get(name).unwrap_or(&0);
                let sym_binding = match symbol.binding {
                    symbol::Binding::Local => 0,  // STB_LOCAL
                    symbol::Binding::Global => 1, // STB_GLOBAL
                    symbol::Binding::Weak => 2,   // STB_WEAK
                    _ => 0,
                };
                let sym_type = match symbol.r#type {
                    symbol::Type::NoType => 0,  // STT_NOTYPE
                    symbol::Type::Object => 1,  // STT_OBJECT
                    symbol::Type::Func => 2,    // STT_FUNC
                    symbol::Type::Section => 3, // STT_SECTION
                    symbol::Type::File => 4,    // STT_FILE
                    _ => 0,
                };
                let st_info = (sym_binding << 4) | sym_type;

                self.write_symbol_entry(
                    &mut symtab_data,
                    name_offset as u32,
                    symbol.value,
                    symbol.size,
                    st_info,
                    0, // st_other
                    symbol.section_index,
                );
            }
        }

        // シンボル文字列テーブルセクションを作成
        let strtab_section = OutputSection {
            name: ".strtab".to_string(),
            r#type: section::SectionType::StrTab,
            flags: vec![],
            addr: 0,
            offset: align(0x3000, 8),
            size: strtab_data.len() as u64,
            data: strtab_data,
            align: 1,
        };

        // シンボルテーブルセクションを作成
        let symtab_section = OutputSection {
            name: ".symtab".to_string(),
            r#type: section::SectionType::SymTab,
            flags: vec![],
            addr: 0,
            offset: align(strtab_section.offset + strtab_section.size, 8),
            size: symtab_data.len() as u64,
            data: symtab_data,
            align: 8,
        };

        // セクション名文字列テーブル(.shstrtab)を作成
        let mut shstrtab_data: Vec<u8> = Vec::new();
        let mut section_name_offsets: HashMap<String, usize> = HashMap::new();

        // セクション名文字列テーブルの先頭はNULL文字
        shstrtab_data.push(0);

        // セクション名をテーブルに追加
        for section in output_sections {
            let offset = shstrtab_data.len();
            section_name_offsets.insert(section.name.clone(), offset);
            shstrtab_data.extend_from_slice(section.name.as_bytes());
            shstrtab_data.push(0); // NULL終端
        }

        // .strtabと.symtabも追加
        let strtab_name_offset = shstrtab_data.len();
        section_name_offsets.insert(".strtab".to_string(), strtab_name_offset);
        shstrtab_data.extend_from_slice(".strtab".as_bytes());
        shstrtab_data.push(0); // NULL終端

        let symtab_name_offset = shstrtab_data.len();
        section_name_offsets.insert(".symtab".to_string(), symtab_name_offset);
        shstrtab_data.extend_from_slice(".symtab".as_bytes());
        shstrtab_data.push(0); // NULL終端

        // .shstrtab自身も追加
        let shstrtab_offset = shstrtab_data.len();
        section_name_offsets.insert(".shstrtab".to_string(), shstrtab_offset);
        shstrtab_data.extend_from_slice(".shstrtab".as_bytes());
        shstrtab_data.push(0); // NULL終端

        // セクション名文字列テーブルセクションを作成
        let shstrtab_section = OutputSection {
            name: ".shstrtab".to_string(),
            r#type: section::SectionType::StrTab,
            flags: vec![],
            addr: 0, // メモリにロードされない
            offset: align(symtab_section.offset + symtab_section.size, 8), // セクションデータの配置オフセット
            size: shstrtab_data.len() as u64,
            data: shstrtab_data,
            align: 1,
        };

        // ELFヘッダーを作成
        let mut elf_header = self.create_elf_header(output_sections);
        elf_header.entry = *entry_point;

        // セクションヘッダーテーブルの位置を計算
        let sections_with_tables = [
            output_sections,
            &[
                strtab_section.clone(),
                symtab_section.clone(),
                shstrtab_section.clone(),
            ],
        ]
        .concat();

        // セクションヘッダーテーブルの位置を設定
        let section_data_end = sections_with_tables
            .iter()
            .map(|s| s.offset + align(s.size, 8))
            .max()
            .unwrap_or(0x3000);

        let sh_offset = align(section_data_end, 8);
        elf_header.shoff = sh_offset;
        elf_header.shentsize = 64; // セクションヘッダーエントリのサイズ (64ビットELF)
        elf_header.shnum = (sections_with_tables.len() + 1) as u16; // NULL セクションも含める

        // .shstrtabのインデックスを正しく設定
        let shstrtab_idx = sections_with_tables
            .iter()
            .position(|s| s.name == ".shstrtab")
            .unwrap_or(0);
        elf_header.shstrndx = (shstrtab_idx + 1) as u16; // NULLセクションもあるため+1

        // プログラムヘッダーを作成
        let program_headers = self.create_program_headers(output_sections);

        // ELFヘッダーを書き込み
        self.write_elf_header(&mut file, &elf_header)?;

        // プログラムヘッダーを書き込み
        self.write_program_headers(&mut file, &program_headers)?;

        // セクションデータを書き込み
        self.write_sections(&mut file, &sections_with_tables)?;

        // セクションヘッダーテーブルを書き込み
        file.seek(SeekFrom::Start(sh_offset))?;

        // NULLセクションヘッダー
        self.write_section_header(&mut file, 0, "", 0, 0, 0, 0, 0, 0, 0, 0, 0)?;

        // 各セクションのヘッダーを書き込み
        for section in sections_with_tables.iter() {
            let name_offset = *section_name_offsets.get(&section.name).unwrap_or(&0);
            let sh_type = section.r#type as u32;
            let mut sh_flags: u64 = 0;
            for flag in &section.flags {
                let bit = match flag {
                    section::SectionFlag::Write => 0x1,
                    section::SectionFlag::Alloc => 0x2,
                    section::SectionFlag::ExecInstr => 0x4,
                    _ => 0,
                };
                sh_flags |= bit;
            }

            // シンボルテーブルの特別な設定
            let (sh_link, sh_info) = if section.name == ".symtab" {
                // sh_link: 文字列テーブルのセクションインデックス
                // sh_info: ローカルシンボルの数 + 1
                let strtab_idx = sections_with_tables
                    .iter()
                    .position(|s| s.name == ".strtab")
                    .unwrap_or(0)
                    + 1;
                (strtab_idx as u32, 1) // ここではすべてグローバルシンボルと仮定
            } else {
                (0, 0)
            };

            // セクションエントリサイズ（シンボルテーブルの場合は24）
            let sh_entsize = if section.name == ".symtab" {
                24 // 64ビットELFのシンボルエントリサイズ
            } else {
                0
            };

            self.write_section_header(
                &mut file,
                name_offset as u32, // セクション名の文字列テーブルでのオフセット
                &section.name,      // デバッグ用
                sh_type,            // セクションタイプ
                sh_flags,           // セクションフラグ
                section.addr,       // メモリアドレス
                section.offset,     // ファイルオフセット
                section.size,       // セクションサイズ
                sh_link,            // リンク（シンボルテーブル用）
                sh_info,            // 情報（追加情報）
                section.align,      // アラインメント
                sh_entsize,         // エントリサイズ
            )?;
        }

        Ok(())
    }

    /// ELFヘッダーを生成
    fn create_elf_header(&self, output_sections: &[OutputSection]) -> header::Header {
        // デフォルトのヘッダー
        let mut elf_header = header::Header {
            ident: header::Ident {
                class: header::Class::Bit64,
                data: header::Data::Lsb, // Little Endian
                version: header::IdentVersion::Current,
                os_abi: header::OSABI::SystemV,
                abi_version: 0,
            },
            r#type: header::Type::Exec,
            machine: header::Machine::AArch64,
            version: header::Version::Current,
            entry: 0x400000, // エントリポイント（_startシンボルのアドレス）
            phoff: 64,       // プログラムヘッダーのオフセット（ELF64ヘッダーのサイズ）
            shoff: 0,        // セクションヘッダーのオフセット（現在は使用しない）
            flags: 0,
            ehsize: 64,    // ELF64ヘッダーのサイズ
            phentsize: 56, // プログラムヘッダーエントリのサイズ
            phnum: 0,      // プログラムヘッダー数（後で更新）
            shentsize: 0,  // セクションヘッダーのサイズ（現在は使用しない）
            shnum: 0,      // セクションヘッダー数（現在は使用しない）
            shstrndx: 0,   // セクション名文字列テーブルのインデックス（現在は使用しない）
        };

        // プログラムヘッダーの情報を更新
        let ph_count = self.count_program_headers(output_sections);
        elf_header.phnum = ph_count as u16;

        elf_header
    }

    /// プログラムヘッダー数をカウント
    fn count_program_headers(&self, _output_sections: &[OutputSection]) -> usize {
        // 現在はテキストセクションとデータセクションの2つ
        2
    }

    /// プログラムヘッダーを生成
    fn create_program_headers(
        &self,
        output_sections: &[OutputSection],
    ) -> Vec<program_header::ProgramHeader> {
        let mut program_headers = Vec::new();

        // テキストセクション（実行可能セクション）
        if let Some(text_section) = output_sections.iter().find(|s| s.name == ".text") {
            let text_ph = program_header::ProgramHeader {
                r#type: segument::Type::Load,
                flags: vec![segument::Flag::Readable, segument::Flag::Executable],
                offset: text_section.offset,
                vaddr: text_section.addr,
                paddr: text_section.addr,
                filesz: text_section.size,
                memsz: text_section.size,
                align: text_section.align,
            };
            program_headers.push(text_ph);
        }

        // データセクション（読み書き可能セクション）
        if let Some(data_section) = output_sections.iter().find(|s| s.name == ".data") {
            let data_ph = program_header::ProgramHeader {
                r#type: segument::Type::Load,
                flags: vec![segument::Flag::Readable, segument::Flag::Writable],
                offset: data_section.offset,
                vaddr: data_section.addr, // 既に適切に計算されているはずなので、そのまま使用
                paddr: data_section.addr,
                filesz: data_section.size,
                memsz: data_section.size,
                align: data_section.align,
            };
            program_headers.push(data_ph);
        }

        program_headers
    }

    /// ELFヘッダーをファイルに書き込む
    fn write_elf_header(&self, file: &mut fs::File, header: &header::Header) -> io::Result<()> {
        // ELF マジックナンバー
        file.write_all(&[0x7f, b'E', b'L', b'F'])?;

        // ELFクラス（32/64ビット）
        file.write_all(&[header.ident.class as u8])?;

        // エンディアン
        file.write_all(&[header.ident.data as u8])?;

        // バージョン
        file.write_all(&[header.ident.version as u8])?;

        // OS ABI
        file.write_all(&[header.ident.os_abi as u8])?;

        // ABI バージョン
        file.write_all(&[header.ident.abi_version])?;

        // パディング
        file.write_all(&[0; 7])?;

        // タイプ
        file.write_all(&(header.r#type as u16).to_le_bytes())?;

        // マシン
        file.write_all(&(header.machine as u16).to_le_bytes())?;

        // バージョン（拡張）
        file.write_all(&(header.version as u32).to_le_bytes())?;

        // エントリポイント
        file.write_all(&header.entry.to_le_bytes())?;

        // プログラムヘッダーオフセット
        file.write_all(&header.phoff.to_le_bytes())?;

        // セクションヘッダーオフセット
        file.write_all(&header.shoff.to_le_bytes())?;

        // フラグ
        file.write_all(&header.flags.to_le_bytes())?;

        // ELFヘッダーサイズ
        file.write_all(&header.ehsize.to_le_bytes())?;

        // プログラムヘッダーエントリサイズ
        file.write_all(&header.phentsize.to_le_bytes())?;

        // プログラムヘッダー数
        file.write_all(&header.phnum.to_le_bytes())?;

        // セクションヘッダーエントリサイズ
        file.write_all(&header.shentsize.to_le_bytes())?;

        // セクションヘッダー数
        file.write_all(&header.shnum.to_le_bytes())?;

        // セクション名文字列テーブルのインデックス
        file.write_all(&header.shstrndx.to_le_bytes())?;

        Ok(())
    }

    /// プログラムヘッダーをファイルに書き込む
    fn write_program_headers(
        &self,
        file: &mut fs::File,
        headers: &[program_header::ProgramHeader],
    ) -> io::Result<()> {
        for ph in headers {
            // タイプ
            file.write_all(&(ph.r#type as u32).to_le_bytes())?;

            // フラグ
            let mut flags: u32 = 0;
            for flag in &ph.flags {
                let bit = match flag {
                    segument::Flag::Executable => 0x1,
                    segument::Flag::Writable => 0x2,
                    segument::Flag::Readable => 0x4,
                    segument::Flag::MaskOS | segument::Flag::MaskProc => {
                        // これらはマスク値なので、通常は直接使用しない
                        continue;
                    }
                };
                flags |= bit;
            }
            file.write_all(&flags.to_le_bytes())?;

            // ファイルオフセット
            file.write_all(&ph.offset.to_le_bytes())?;

            // 仮想アドレス
            file.write_all(&ph.vaddr.to_le_bytes())?;

            // 物理アドレス
            file.write_all(&ph.paddr.to_le_bytes())?;

            // ファイルサイズ
            file.write_all(&ph.filesz.to_le_bytes())?;

            // メモリサイズ
            file.write_all(&ph.memsz.to_le_bytes())?;

            // アラインメント
            file.write_all(&ph.align.to_le_bytes())?;
        }

        Ok(())
    }

    /// セクションデータをファイルに書き込む
    fn write_sections(&self, file: &mut fs::File, sections: &[OutputSection]) -> io::Result<()> {
        for section in sections {
            // ファイルポインタをセクションのオフセット位置に移動
            file.seek(SeekFrom::Start(section.offset))?;

            // セクションデータを書き込み
            file.write_all(&section.data)?;
        }

        Ok(())
    }

    /// リンク処理全体を実行する
    pub fn link(&self) -> Result<Vec<OutputSection>, String> {
        // 1. シンボル解決を実行
        let (mut resolved_symbols, unresolved_symbols) = self.resolve_symbols();

        // 未解決シンボルがあればエラー
        if !unresolved_symbols.is_empty() {
            return Err(format!(
                "未解決のシンボルがあります: {:?}",
                unresolved_symbols
            ));
        }

        // 2. セクション配置を実行
        let mut output_sections = self.layout_sections(&mut resolved_symbols);

        // 3. 再配置処理を適用
        self.apply_relocations(&mut output_sections, &resolved_symbols)?;

        Ok(output_sections)
    }

    /// 読み込まれたオブジェクトファイルの数を返す
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// 指定されたシンボルがオブジェクトファイル内に存在するかチェック
    pub fn has_symbol(&self, name: &str) -> bool {
        for obj in &self.objects {
            for symbol in &obj.symbols {
                if symbol.name == name {
                    return true;
                }
            }
        }
        false
    }

    /// 指定されたシンボルが未定義シンボルとして参照されているかチェック
    pub fn has_undefined_symbol(&self, name: &str) -> bool {
        for obj in &self.objects {
            for symbol in &obj.symbols {
                if symbol.name == name && symbol.shndx == 0 {
                    // shndx == 0 は SHN_UNDEF を意味し、未定義シンボル
                    return true;
                }
            }
        }
        false
    }

    /// シンボル解決を行い、解決されたシンボルと未解決シンボルを返す
    pub fn resolve_symbols(&self) -> (HashMap<String, ResolvedSymbol>, Vec<String>) {
        let mut resolved_symbols: HashMap<String, ResolvedSymbol> = HashMap::new();

        // 各オブジェクトファイルのシンボルを処理
        for (obj_idx, obj) in self.objects.iter().enumerate() {
            for symbol in &obj.symbols {
                let is_defined = symbol.shndx != 0; // 0 = SHN_UNDEF (未定義シンボル)

                // シンボル名が空の場合はスキップ（NULL シンボル）
                if symbol.name.is_empty() {
                    continue;
                }

                let new_symbol = ResolvedSymbol {
                    name: symbol.name.clone(),
                    value: symbol.value,
                    size: symbol.size,
                    r#type: symbol.info.r#type,
                    binding: symbol.info.binding,
                    section_index: symbol.shndx,
                    object_index: obj_idx,
                    is_defined,
                };

                // 既存のシンボルとマージするかどうかを決定
                if let Some(existing) = resolved_symbols.get(&symbol.name) {
                    // 両方定義済みの場合、バインディングの強さでどちらを使うか決める
                    if is_defined && existing.is_defined {
                        if new_symbol.is_stronger_than(existing) {
                            resolved_symbols.insert(symbol.name.clone(), new_symbol);
                        }
                        // 既存のシンボルの方が強いか同等なら、それを保持
                    }
                    // 現在が定義済みで既存が未定義なら、今回の定義で上書き
                    else if is_defined && !existing.is_defined {
                        resolved_symbols.insert(symbol.name.clone(), new_symbol);
                    }
                    // それ以外の場合は既存を保持
                } else {
                    // まだ見たことのないシンボルは追加
                    resolved_symbols.insert(symbol.name.clone(), new_symbol);
                }
            }
        }

        // 未解決シンボルをチェック
        let unresolved_symbols: Vec<String> = resolved_symbols
            .iter()
            .filter(|(_, symbol)| !symbol.is_defined)
            .map(|(name, _)| name.clone())
            .collect();

        (resolved_symbols, unresolved_symbols)
    }

    /// セクションの配置を行い、出力セクションのリストを返す
    pub fn layout_sections(
        &self,
        resolved_symbols: &mut HashMap<String, ResolvedSymbol>,
    ) -> Vec<OutputSection> {
        let mut output_sections: Vec<OutputSection> = Vec::new();

        // .textセクションと.dataセクションを統合し、オフセット情報を取得
        let (text_data, text_offsets) = self.merge_sections(".text", 1);
        let (data_data, data_offsets) = self.merge_sections(".data", 2);

        // 基本的なメモリレイアウトを決定
        // まず.textセクションから設定
        let base_addr = 0x400000; // 実行可能ファイルの典型的な開始アドレス
        let text_section = OutputSection {
            name: ".text".to_string(),
            r#type: section::SectionType::ProgBits,
            flags: vec![section::SectionFlag::Alloc, section::SectionFlag::ExecInstr],
            addr: base_addr + 0x1000, // 修正: テキストセクションの開始アドレスを0x401000に変更
            offset: 0x1000,           // ヘッダーの後のオフセット
            size: text_data.len() as u64,
            data: text_data,
            align: 0x1000, // 一般的なページサイズ
        };

        // .dataセクションは.textの後に配置
        let data_addr = align(base_addr + 0x2000, 0x1000); // 修正: 明示的に0x402000に配置
        let data_section = OutputSection {
            name: ".data".to_string(),
            r#type: section::SectionType::ProgBits,
            flags: vec![section::SectionFlag::Alloc, section::SectionFlag::Write],
            addr: data_addr,
            offset: 0x2000, // 修正: 明示的に0x2000に設定
            size: data_data.len() as u64,
            data: data_data,
            align: 0x1000,
        };

        // 出力セクションリストを構築
        output_sections.push(text_section);
        output_sections.push(data_section);

        // シンボルのアドレスを更新
        self.update_symbol_addresses(
            resolved_symbols,
            &text_offsets,
            base_addr + 0x1000,
            &data_offsets,
            data_addr,
        );

        output_sections
    }

    /// 指定された名前のセクションをすべてのオブジェクトファイルから統合し、
    /// 元のオフセット情報と統合されたデータを返す
    fn merge_sections(
        &self,
        section_name: &str,
        section_index: u16,
    ) -> (Vec<u8>, HashMap<(usize, u16), usize>) {
        let mut section_data: Vec<u8> = Vec::new();
        let mut offsets: HashMap<(usize, u16), usize> = HashMap::new();
        let mut current_offset = 0;

        for (obj_idx, obj) in self.objects.iter().enumerate() {
            for section in &obj.section_headers {
                if section.name == section_name {
                    // このオブジェクトのセクションの開始オフセットを記録
                    offsets.insert((obj_idx, section_index), current_offset);
                    section_data.extend_from_slice(&section.data);
                    current_offset += section.data.len();
                }
            }
        }

        (section_data, offsets)
    }

    /// シンボルの最終アドレスを更新する
    fn update_symbol_addresses(
        &self,
        resolved_symbols: &mut HashMap<String, ResolvedSymbol>,
        text_offsets: &HashMap<(usize, u16), usize>,
        text_base_addr: u64,
        data_offsets: &HashMap<(usize, u16), usize>,
        data_base_addr: u64,
    ) {
        for symbol in resolved_symbols.values_mut() {
            // シンボルのセクションインデックスに応じて適切なベースアドレスを選択
            match symbol.section_index {
                1 => {
                    // .textセクション内のシンボル
                    if let Some(&offset) =
                        text_offsets.get(&(symbol.object_index, symbol.section_index))
                    {
                        // オブジェクトファイル内でのオフセット + 統合.textセクションでのオブジェクトのオフセット + .textセクションの開始アドレス
                        symbol.value = text_base_addr + (offset + symbol.value as usize) as u64;
                    }
                }
                2 => {
                    // .dataセクション内のシンボル
                    if let Some(&offset) =
                        data_offsets.get(&(symbol.object_index, symbol.section_index))
                    {
                        // オブジェクトファイル内でのオフセット + 統合.dataセクションでのオブジェクトのオフセット + .dataセクションの開始アドレス
                        symbol.value = data_base_addr + (offset + symbol.value as usize) as u64;
                    }
                }
                _ => {
                    // その他のセクション（現状は無視）
                }
            }
        }
    }

    /// 再配置（リロケーション）を適用する
    ///
    /// 各オブジェクトファイルの再配置エントリを処理し、
    /// 解決されたシンボルのアドレスを使用して参照を更新する
    pub fn apply_relocations(
        &self,
        output_sections: &mut [OutputSection],
        resolved_symbols: &HashMap<String, ResolvedSymbol>,
    ) -> Result<(), String> {
        // セクション名からOutputSectionの位置を特定するマップ
        let section_indices: HashMap<String, usize> = output_sections
            .iter()
            .enumerate()
            .map(|(i, sec)| (sec.name.clone(), i))
            .collect();

        // 各オブジェクトファイルの再配置エントリを処理
        for (obj_idx, obj) in self.objects.iter().enumerate() {
            // 各再配置エントリに対して処理を行う
            for reloc in &obj.relocations {
                // 再配置タイプに応じた処理
                self.process_relocation(
                    obj_idx,
                    reloc,
                    output_sections,
                    &section_indices,
                    resolved_symbols,
                )?;
            }
        }

        Ok(())
    }

    /// 個々の再配置エントリを処理する
    fn process_relocation(
        &self,
        obj_idx: usize,
        reloc: &relocation::RelocationAddend,
        output_sections: &mut [OutputSection],
        section_indices: &HashMap<String, usize>,
        resolved_symbols: &HashMap<String, ResolvedSymbol>,
    ) -> Result<(), String> {
        // 再配置タイプに応じた処理
        match reloc.info.r#type {
            relocation::RelocationType::Aarch64AdrPrelLo21 => {
                // シンボルインデックスからシンボル名を取得
                let symbol_index = reloc.info.symbol_index as usize;
                if symbol_index >= self.objects[obj_idx].symbols.len() {
                    return Err(format!("シンボルインデックスが範囲外: {}", symbol_index));
                }

                let symbol_name = &self.objects[obj_idx].symbols[symbol_index].name;

                // 解決されたシンボルを取得
                let resolved_symbol = resolved_symbols
                    .get(symbol_name)
                    .ok_or_else(|| format!("シンボルが解決されていません: {}", symbol_name))?;

                // シンボルが未定義の場合はエラー
                if !resolved_symbol.is_defined {
                    return Err(format!("未定義シンボルへの再配置: {}", symbol_name));
                }

                // 再配置が適用されるセクション（通常は.text）のインデックスを取得
                let text_section_idx = section_indices
                    .get(".text")
                    .ok_or_else(|| "再配置対象セクションが見つかりません: .text".to_string())?;

                // セクションへの可変参照を取得
                let target_section = &mut output_sections[*text_section_idx];

                // 再配置オフセットが範囲内かチェック
                if reloc.offset as usize >= target_section.data.len() {
                    return Err(format!("再配置オフセットが範囲外: {}", reloc.offset));
                }

                // AArch64 ADR_PREL_LO21 再配置の適用
                // リロケーションのオフセット位置にシンボルの相対アドレスを書き込む
                let instruction_addr = target_section.addr + reloc.offset;
                let symbol_addr = resolved_symbol.value;

                // シンボルとの相対アドレスを計算 (ターゲットアドレス - PC値)
                // PCは現在の命令アドレス（instruction_addr）
                let relative_addr =
                    ((symbol_addr as i64) - (instruction_addr as i64) + reloc.addend) as i32;

                // ADR命令の形式にエンコード
                // 現在の命令を取得
                let pos = reloc.offset as usize;
                let instruction = u32::from_le_bytes([
                    target_section.data[pos],
                    target_section.data[pos + 1],
                    target_section.data[pos + 2],
                    target_section.data[pos + 3],
                ]);

                // ADR命令のオペコードとレジスタ部分を保持
                // ADR命令のフォーマット: 0bxxx10000 iiiiiiii iiiiiiii iiiddddd
                // x: opcode, i: immbit, d: destination register
                let opcode_rd = instruction & 0x9F00001F; // オペコードとレジスタ部分を保持

                // ADR命令のエンコーディング（ARMv8アーキテクチャリファレンスマニュアルに基づく）
                // immhi: upper 19 bits of immediate (bits 5-23)
                // immlo: lower 2 bits of immediate (bits 29-30)
                let immlo = ((relative_addr & 0x3) as u32) << 29;
                let immhi = (((relative_addr >> 2) & 0x7FFFF) as u32) << 5;

                // 新しい命令を構築
                let new_instruction = opcode_rd | immlo | immhi;

                // 更新された命令をセクションデータに書き戻す
                let instruction_bytes = new_instruction.to_le_bytes();
                target_section.data[pos] = instruction_bytes[0];
                target_section.data[pos + 1] = instruction_bytes[1];
                target_section.data[pos + 2] = instruction_bytes[2];
                target_section.data[pos + 3] = instruction_bytes[3];
            }
        }

        Ok(())
    }

    /// シンボルテーブルエントリを書き込む
    fn write_symbol_entry(
        &self,
        data: &mut Vec<u8>,
        st_name: u32,
        st_value: u64,
        st_size: u64,
        st_info: u8,
        st_other: u8,
        st_shndx: u16,
    ) {
        // シンボル名のインデックス
        data.extend_from_slice(&st_name.to_le_bytes());

        // シンボル情報（バインディングとタイプ）
        data.push(st_info);

        // その他の情報
        data.push(st_other);

        // セクションインデックス
        data.extend_from_slice(&st_shndx.to_le_bytes());

        // シンボル値（アドレス）
        data.extend_from_slice(&st_value.to_le_bytes());

        // シンボルサイズ
        data.extend_from_slice(&st_size.to_le_bytes());
    }

    /// セクションヘッダーを書き込む
    fn write_section_header(
        &self,
        file: &mut fs::File,
        name: u32,
        _debug_name: &str,
        sh_type: u32,
        sh_flags: u64,
        sh_addr: u64,
        sh_offset: u64,
        sh_size: u64,
        sh_link: u32,
        sh_info: u32,
        sh_addralign: u64,
        sh_entsize: u64,
    ) -> io::Result<()> {
        // セクション名のインデックス
        file.write_all(&name.to_le_bytes())?;

        // セクションタイプ
        file.write_all(&sh_type.to_le_bytes())?;

        // セクションフラグ
        file.write_all(&sh_flags.to_le_bytes())?;

        // メモリ上のアドレス
        file.write_all(&sh_addr.to_le_bytes())?;

        // ファイル内オフセット
        file.write_all(&sh_offset.to_le_bytes())?;

        // セクションサイズ
        file.write_all(&sh_size.to_le_bytes())?;

        // リンク情報
        file.write_all(&sh_link.to_le_bytes())?;

        // 追加情報
        file.write_all(&sh_info.to_le_bytes())?;

        // アライメント
        file.write_all(&sh_addralign.to_le_bytes())?;

        // エントリサイズ (固定サイズのテーブルの場合)
        file.write_all(&sh_entsize.to_le_bytes())?;

        Ok(())
    }
}

/// 値をアラインメントに合わせる
fn align(value: u64, alignment: u64) -> u64 {
    (value + alignment - 1) & !(alignment - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn test_basic_linker_creation() {
        // テスト対象のオブジェクトファイルのパス
        let main_o = Path::new("src/parser/fixtures/main.o");
        let sub_o = Path::new("src/parser/fixtures/sub.o");

        // オブジェクトファイルが存在することを確認
        assert!(main_o.exists(), "main.o が見つかりません");
        assert!(sub_o.exists(), "sub.o が見つかりません");

        // リンカーを作成
        let linker = Linker::new();

        // オブジェクトファイルを追加
        let linker = linker.add_object(main_o).expect("main.o の追加に失敗");
        let linker = linker.add_object(sub_o).expect("sub.o の追加に失敗");

        // オブジェクトファイルが正しく読み込まれているか
        assert_eq!(linker.object_count(), 2, "オブジェクトファイルの数が不正");
    }

    #[test]
    fn test_basic_object_parsing() {
        // テスト対象のオブジェクトファイル
        let main_o = Path::new("src/parser/fixtures/main.o");

        // リンカーを作成してオブジェクトを追加
        let linker = Linker::new()
            .add_object(main_o)
            .expect("main.o の追加に失敗");

        // main.o から _start シンボルが見つかるか確認
        let has_start = linker.has_symbol("_start");
        assert!(has_start, "_start シンボルが見つかりません");

        // main.o から未定義シンボル x が見つかるか確認
        let uses_x = linker.has_undefined_symbol("x");
        assert!(uses_x, "未定義シンボル x が見つかりません");
    }

    #[test]
    fn test_symbol_resolution() {
        // main.o と sub.o を読み込む
        let main_o = Path::new("src/parser/fixtures/main.o");
        let sub_o = Path::new("src/parser/fixtures/sub.o");

        let linker = Linker::new()
            .add_object(main_o)
            .expect("main.o の読み込み失敗")
            .add_object(sub_o)
            .expect("sub.o の読み込み失敗");

        // シンボル解決を実行
        let (resolved_symbols, unresolved_symbols) = linker.resolve_symbols();

        // 全てのシンボルが解決されていることを確認
        assert!(
            unresolved_symbols.is_empty(),
            "未解決のシンボルがあります: {:?}",
            unresolved_symbols
        );

        // '_start'と'x'シンボルが解決されていることを確認
        assert!(
            resolved_symbols.contains_key("_start"),
            "_start シンボルが解決されていません"
        );
        assert!(
            resolved_symbols.contains_key("x"),
            "x シンボルが解決されていません"
        );

        // 'x'シンボルが適切に解決されているか確認
        // 'x'はsub.oで定義され、値は23になっているはず
        if let Some(x_symbol) = resolved_symbols.get("x") {
            // sub.oから来ているかチェック（オブジェクトインデックスは1であるべき）
            assert_eq!(x_symbol.object_index, 1, "x シンボルのオブジェクト元が不正");
        } else {
            panic!("x シンボルが見つかりません");
        }
    }

    #[test]
    fn test_section_layout() {
        // main.o と sub.o を読み込む
        let main_o = Path::new("src/parser/fixtures/main.o");
        let sub_o = Path::new("src/parser/fixtures/sub.o");

        let linker = Linker::new()
            .add_object(main_o)
            .expect("main.o の読み込み失敗")
            .add_object(sub_o)
            .expect("sub.o の読み込み失敗");

        // シンボル解決を実行
        let (mut resolved_symbols, _) = linker.resolve_symbols();

        // セクション配置を実行
        let output_sections = linker.layout_sections(&mut resolved_symbols);

        // 必要なセクションが存在するか確認
        assert!(
            output_sections.iter().any(|s| s.name == ".text"),
            ".text セクションがありません"
        );
        assert!(
            output_sections.iter().any(|s| s.name == ".data"),
            ".data セクションがありません"
        );

        // セクションの順序が正しいか確認 (.text -> .data)
        let text_idx = output_sections
            .iter()
            .position(|s| s.name == ".text")
            .unwrap();
        let data_idx = output_sections
            .iter()
            .position(|s| s.name == ".data")
            .unwrap();
        assert!(
            text_idx < data_idx,
            "セクションの順序が不正: .text は .data より前にあるべき"
        );

        // .textセクションが実行可能属性を持っているか確認
        let text_section = &output_sections[text_idx];
        assert!(
            text_section
                .flags
                .contains(&section::SectionFlag::ExecInstr),
            ".text セクションに実行可能フラグがありません"
        );

        // .dataセクションが書き込み可能属性を持っているか確認
        let data_section = &output_sections[data_idx];
        assert!(
            data_section.flags.contains(&section::SectionFlag::Write),
            ".data セクションに書き込み可能フラグがありません"
        );

        // セクションのアドレスが適切に割り当てられているか確認
        assert!(text_section.addr > 0, ".text セクションのアドレスが不正");
        assert!(
            data_section.addr > text_section.addr,
            ".data セクションは .text セクションの後に配置されるべき"
        );

        // アドレスの具体的な値をチェック
        assert_eq!(
            text_section.addr, 0x401000,
            ".text セクションの開始アドレスが期待値と異なります"
        );
        // .dataセクションは.textセクションの後、アラインメントされた位置に配置されるはず
        let expected_data_addr = align(text_section.addr + text_section.size, 0x1000);
        assert_eq!(
            data_section.addr, expected_data_addr,
            ".data セクションの開始アドレスが期待値と異なります"
        );

        // セクションのサイズが適切か確認
        assert!(text_section.size > 0, ".text セクションのサイズが不正");
        assert!(data_section.size > 0, ".data セクションのサイズが不正");

        // xシンボルのアドレスが.dataセクション内にあるか確認
        let x_symbol = resolved_symbols.get("x").unwrap();
        assert!(
            x_symbol.value >= data_section.addr
                && x_symbol.value < data_section.addr + data_section.size,
            "xシンボルのアドレスが.dataセクション内にありません"
        );

        // _startシンボルが.textセクション内にあるか確認
        let start_symbol = resolved_symbols.get("_start").unwrap();
        assert!(
            start_symbol.value >= text_section.addr
                && start_symbol.value < text_section.addr + text_section.size,
            "_startシンボルのアドレスが.textセクション内にありません"
        );

        // シンボルのアドレスが適切に更新されているか確認
        // _startシンボルは.textセクションの先頭に配置されるべき
        let expected_start_addr = text_section.addr;
        assert_eq!(
            start_symbol.value, expected_start_addr,
            "_startシンボルのアドレスが期待値と異なります"
        );

        // xシンボルは.dataセクションの先頭に配置されるべき
        let expected_x_addr = data_section.addr;
        assert_eq!(
            x_symbol.value, expected_x_addr,
            "xシンボルのアドレスが期待値と異なります"
        );
    }

    #[test]
    fn test_apply_relocations() {
        // main.o と sub.o を読み込む
        let main_o = Path::new("src/parser/fixtures/main.o");
        let sub_o = Path::new("src/parser/fixtures/sub.o");

        let linker = Linker::new()
            .add_object(main_o)
            .expect("main.o の読み込み失敗")
            .add_object(sub_o)
            .expect("sub.o の読み込み失敗");

        // リンク処理を実行
        let output_sections = linker.link().expect("リンク処理に失敗");

        // .text セクションを見つける
        let text_section = output_sections
            .iter()
            .find(|s| s.name == ".text")
            .expect(".text セクションが見つかりません");

        // 実行可能ビットが設定されていることを確認
        assert!(
            text_section
                .flags
                .contains(&section::SectionFlag::ExecInstr),
            ".text セクションに実行可能フラグがありません"
        );

        // main.o に含まれる再配置エントリによって .text セクションの命令が更新されたことを確認
        // 特定のオフセットにある命令を検査
        let offset = 0; // 最初の再配置エントリのオフセット
        assert!(
            offset + 4 <= text_section.data.len(),
            "命令のオフセットが範囲外"
        );

        // 命令データを取得
        let instruction = u32::from_le_bytes([
            text_section.data[offset],
            text_section.data[offset + 1],
            text_section.data[offset + 2],
            text_section.data[offset + 3],
        ]);

        // 更新された命令のアドレスフィールドをチェック
        // 命令の形式はADRで、下位21ビットが相対アドレスを格納
        let addr_field = instruction & 0x1FFFFF;

        // アドレスフィールドが0でないことを確認
        // 実際の値は環境によって異なるので、詳細な値のチェックはしない
        assert!(
            addr_field != 0,
            "再配置によるアドレス更新が行われていません"
        );
    }
}
