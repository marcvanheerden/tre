mod grow;
use clap::Parser;

/// A basic tree cli tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path for tree to run from, defaults to .
    #[arg(default_value = ".")]
    path: String,
    #[arg(short = 'd', long = "depth", default_value = "5")]
    max_depth: usize,
}

fn main() {
    let args = Args::parse();
    println!("{}", grow::tre(args.path, args.max_depth));
}
