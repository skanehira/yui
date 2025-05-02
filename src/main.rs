use std::env;
use std::io::Write as _;
use std::path::Path;
use std::process;

use yui::linker::Linker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <output> <input1> [<input2> ...]", args[0]);
        process::exit(1);
    }

    let input_paths: Vec<&Path> = args[2..].iter().map(Path::new).collect();
    let inputs = {
        let mut inputs = Vec::with_capacity(input_paths.len());
        for path in input_paths.iter() {
            let data = std::fs::read(path)?;
            inputs.push(data);
        }
        inputs
    };

    let mut linker = Linker::new();

    let mut out = create_output_file(Path::new(&args[1]))?;
    out.write_all(&linker.link_to_file(inputs)?)?;
    Ok(())
}

fn create_output_file(path: &Path) -> Result<std::fs::File, std::io::Error> {
    use std::os::unix::fs::OpenOptionsExt as _;

    let mut options = std::fs::OpenOptions::new();
    let file = options
        .write(true)
        .truncate(true)
        .create(true)
        .mode(0o655) // rw-r-xr-x
        .open(path)?;

    Ok(file)
}
