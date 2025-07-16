use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run without interactive mode
    #[arg(short, long)]
    silent: bool,

    /// Path to the directory to scan
    #[arg(default_value = ".")]
    path: String,
}


fn main() {
    let cli = Cli::parse();
    println!("Silent mode: {}", cli.silent);
    println!("Path: {}", cli.path);
}
