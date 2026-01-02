# Yui リンカー アーキテクチャ改善提案書

## エグゼクティブサマリー

Yuiは教育目的で作成されたELFリンカーとして、基本的な機能は実装されていますが、プロダクションレベルの品質に向けて以下の改善点があります。本文書では、アーキテクチャ解析の結果に基づき、コード品質、パフォーマンス、機能性の観点から具体的な改善提案を行います。

## 1. アーキテクチャレベルの改善

### 1.1 エラーハンドリングの統一化

**現状の問題点:**
- `Box<dyn std::error::Error>`による型消去でエラーの詳細情報が失われる
- パーサーエラーとリンカーエラーの区別が不明確
- エラーメッセージがフォーマット文字列で生成されており、構造化されていない

**改善提案:**
```rust
// src/error.rs を新規作成
#[derive(Debug, thiserror::Error)]
pub enum LinkerError {
    #[error("Duplicate symbol: {name} (first defined in {first}, redefined in {second})")]
    DuplicateSymbol {
        name: String,
        first: String,
        second: String,
    },
    
    #[error("Unresolved symbols: {symbols:?}")]
    UnresolvedSymbols { symbols: Vec<String> },
    
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Section '{name}' not found")]
    SectionNotFound { name: String },
}
```

### 1.2 メモリ効率の改善

**現状の問題点:**
- シンボル名の頻繁な`clone()`操作（resolve_symbols内で5回以上）
- セクションデータの重複保持
- 大きなバイナリに対するスケーラビリティの欠如

**改善提案:**
```rust
// インターンされた文字列を使用
use string_interner::{StringInterner, Symbol};

pub struct SymbolTable {
    interner: StringInterner,
    symbols: HashMap<Symbol, ResolvedSymbol>,
}

// Cow (Copy-on-Write) を活用
use std::borrow::Cow;

pub struct Section<'a> {
    name: Cow<'a, str>,
    data: Cow<'a, [u8]>,
    // ...
}
```

### 1.3 モジュール構造の再編成

**現状の問題点:**
- `linker.rs`が700行を超える巨大ファイル
- 責務の分離が不十分
- プライベートメソッドと公開メソッドの混在

**改善提案:**
```
src/
├── linker/
│   ├── mod.rs          // Linker構造体とpublicインターフェース
│   ├── symbol.rs       // シンボル解決ロジック
│   ├── section.rs      // セクションレイアウトロジック
│   ├── relocation.rs   // リロケーション処理
│   ├── output.rs       // 出力生成ロジック
│   └── writer.rs       // バイナリ書き込みヘルパー
```

## 2. 機能の拡張

### 2.1 リロケーションタイプの拡充

**現状の問題点:**
- `AARCH64_ADR_PREL_LO21`のみサポート
- 他の一般的なリロケーションタイプが未実装

**改善提案:**
```rust
impl Linker {
    fn process_relocation(&self, reloc: &RelocationAddend) -> Result<(), LinkerError> {
        match reloc.info.r#type {
            RelocationType::Aarch64AdrPrelLo21 => self.process_adr_prel_lo21(reloc),
            RelocationType::Aarch64Call26 => self.process_call26(reloc),
            RelocationType::Aarch64Jump26 => self.process_jump26(reloc),
            RelocationType::Aarch64Abs64 => self.process_abs64(reloc),
            RelocationType::Aarch64Abs32 => self.process_abs32(reloc),
            _ => Err(LinkerError::UnsupportedRelocation(reloc.info.r#type)),
        }
    }
}
```

### 2.2 デバッグ情報のサポート

**現状の問題点:**
- デバッグセクション（.debug_*）が無視される
- 出力ファイルにデバッグ情報が含まれない

**改善提案:**
- DWARF形式のデバッグ情報の保持と転送
- ソースレベルデバッグのサポート

### 2.3 動的リンクの基礎実装

**現状の問題点:**
- 静的リンクのみサポート
- 共有ライブラリの作成・リンクが不可能

**改善提案:**
- PLT/GOTセクションの生成
- 動的シンボルテーブルの作成
- インタープリタセクションの追加

## 3. パフォーマンス最適化

### 3.1 並列処理の導入

**現状の問題点:**
- シンボル解決が逐次処理
- 複数オブジェクトファイルの解析が直列

**改善提案:**
```rust
use rayon::prelude::*;

pub fn resolve_symbols_parallel(&self) -> Result<HashMap<String, ResolvedSymbol>, LinkerError> {
    let symbols: Vec<_> = self.objects
        .par_iter()
        .enumerate()
        .flat_map(|(idx, obj)| {
            obj.symbols.iter().map(move |sym| (idx, sym))
        })
        .collect();
    
    // 並列でシンボルを処理
}
```

### 3.2 インクリメンタルリンキング

**現状の問題点:**
- 毎回フルリンクが必要
- 変更されていないオブジェクトも再処理

**改善提案:**
- オブジェクトファイルのハッシュによる変更検知
- 差分リンキングの実装

## 4. コード品質の改善

### 4.1 テストカバレッジの向上

**現状の問題点:**
- エラーケースのテストが不足
- エッジケースの検証が不十分
- E2Eテストがシェルスクリプトベース

**改善提案:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_duplicate_weak_symbols() {
        // 弱シンボルの重複解決テスト
    }
    
    #[test]
    fn test_circular_dependencies() {
        // 循環依存の検出テスト
    }
    
    #[test]
    fn test_large_binary_handling() {
        // 大規模バイナリのテスト
    }
}
```

### 4.2 ドキュメンテーションの充実

**現状の問題点:**
- 内部アーキテクチャのドキュメント不足
- ELFフォーマットの説明が不十分

**改善提案:**
- 各モジュールに詳細なドキュメントコメント追加
- アーキテクチャ決定記録（ADR）の導入
- 図解によるデータフローの可視化

### 4.3 型安全性の向上

**現状の問題点:**
- マジックナンバーの使用（例：`0x400000`）
- 数値型の混在（`u16`, `u32`, `u64`）

**改善提案:**
```rust
// 型安全なアドレス表現
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualAddress(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileOffset(u64);

const BASE_ADDRESS: VirtualAddress = VirtualAddress(0x400000);
```

## 5. 開発者体験の向上

### 5.1 CLIの改善

**現状の問題点:**
- 基本的な引数パースのみ
- 詳細なオプションがない

**改善提案:**
```rust
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Output file path
    #[arg(short, long)]
    output: PathBuf,
    
    /// Input object files
    inputs: Vec<PathBuf>,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Generate map file
    #[arg(long)]
    map: Option<PathBuf>,
}
```

### 5.2 診断メッセージの改善

**現状の問題点:**
- エラー位置の特定が困難
- 修正方法の提案がない

**改善提案:**
- ソースコード位置の追跡
- 類似シンボル名の提案（typoの可能性）
- カラー出力によるエラーの視認性向上

## 6. セキュリティの考慮

### 6.1 入力検証の強化

**現状の問題点:**
- 悪意のあるELFファイルに対する脆弱性
- バッファオーバーフローの可能性

**改善提案:**
- 入力サイズの制限
- セクションオフセットの境界チェック
- スタックカナリーの実装

## 実装優先順位

1. **高優先度（短期）**
   - エラーハンドリングの改善
   - メモリ効率の最適化
   - テストカバレッジの向上

2. **中優先度（中期）**
   - リロケーションタイプの拡充
   - モジュール構造の再編成
   - CLIの改善

3. **低優先度（長期）**
   - 動的リンクのサポート
   - デバッグ情報の処理
   - インクリメンタルリンキング

## まとめ

Yuiは教育目的のプロジェクトとして優れた実装ですが、上記の改善を行うことで、より実用的で保守性の高いリンカーに進化できます。特に、エラーハンドリングとメモリ効率の改善は、コードの品質向上に直接的に寄与するため、優先的に取り組むべきです。