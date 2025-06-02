mod grow;
use clap::Parser;

const DEFAULT_PATH: &str = ".";
const DEFAULT_DEPTH: &str = "5";

/// A basic tree cli tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path for tree to run from, defaults to .
    #[arg(default_value = DEFAULT_PATH)]
    path: String,
    #[arg(short = 'd', long = "depth", default_value = DEFAULT_DEPTH)]
    max_depth: usize,
}

fn main() {
    let args = Args::parse();

    // allow the user to use a shorthand `tre 4` to get depth of 4
    if args.max_depth == DEFAULT_DEPTH.parse().unwrap()
        && let Ok(depth) = args.path.parse::<usize>()
    {
        println!("{}", grow::tre(DEFAULT_PATH.into(), depth));
        return;
    }

    println!("{}", grow::tre(args.path, args.max_depth));
}
