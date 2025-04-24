use std::env;
use std::path::Path;
use std::process;

use yui::linker::Linker;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "使用方法: {} <出力ファイル> <入力オブジェクトファイル1> [<入力オブジェクトファイル2> ...]",
            args[0]
        );
        process::exit(1);
    }

    let output_path = Path::new(&args[1]);
    let input_paths: Vec<&Path> = args[2..].iter().map(Path::new).collect();

    // リンカーを初期化
    let mut linker = Linker::new();

    // 入力オブジェクトファイルを追加
    for input_path in input_paths {
        match linker.add_object(input_path) {
            Ok(l) => {
                linker = l;
                println!("オブジェクトファイルを追加: {}", input_path.display());
            }
            Err(e) => {
                eprintln!("エラー: {} の読み込みに失敗: {}", input_path.display(), e);
                process::exit(1);
            }
        }
    }

    // リンク処理を実行し、出力ファイルを生成
    match linker.link_to_file(output_path) {
        Ok(_) => {
            println!("リンク成功: {}", output_path.display());
        }
        Err(e) => {
            eprintln!("リンクエラー: {}", e);
            process::exit(1);
        }
    }
}
