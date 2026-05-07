use clap::Parser;
use rubik::{analyze_path, output};
use std::process;

#[derive(Parser)]
#[command(name = "rubik", version)]
struct Args {
    /// Path to a file or directory to analyze
    path: std::path::PathBuf,

    /// Output format: pretty or json
    #[arg(short, long, default_value = "pretty")]
    format: String,
}

fn main() {
    let args = Args::parse();

    let results = match analyze_path(&args.path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    for file in &results {
        if let Some(ref err) = file.error {
            eprintln!("Error parsing {}: {}", file.path.display(), err);
        }
    }

    let formatter = output::get_formatter(&args.format);
    println!("{}", formatter.format(&results));
}
