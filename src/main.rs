use std::env;
use std::path::Path;
use std::process;

use yui::linker::Linker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <output> <input1> [<input2> ...]", args[0]);
        process::exit(1);
    }

    let output_path = Path::new(&args[1]);
    let input_paths: Vec<&Path> = args[2..].iter().map(Path::new).collect();

    let mut linker = Linker::new();

    linker.link_to_file(output_path, &input_paths)?;
    Ok(())
}
