use clap::Parser;
use fibora::fibonacci;

#[derive(Debug, Parser)]
#[clap(author = "Eliaz Bobadilla", version = "1.2.0", about = "Fibonacci CLI Utilities.", long_about = None)]
struct Cli {
    number: usize,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", fibonacci(cli.number));
}
