use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    ipa: String
}

fn main() {
    let ipa = Cli::parse().ipa;

    println!("{}", ipa);
}
