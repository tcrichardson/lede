use clap::Parser;
use lede::{analyze_path, output};
use std::process;

#[derive(Parser)]
#[command(name = "lede", version)]
struct Args {
    /// Path to a file or directory to analyze
    path: std::path::PathBuf,

    /// Output format: markdown, pretty, or json
    #[arg(short, long, default_value = "markdown")]
    format: String,

    /// Include closures and lambda expressions in the analysis
    #[arg(long)]
    include_closures: bool,
}

fn main() {
    let args = Args::parse();

    let results = match analyze_path(&args.path, args.include_closures) {
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

    let clusters = lede::duplicates::compute_duplicates(&results);
    let formatter = output::get_formatter(&args.format);
    println!("{}", formatter.format(&results, &clusters));
}
