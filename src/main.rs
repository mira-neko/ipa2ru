mod lib;

use clap::Parser;
use lib::{RuPhonemeSec, IPA};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    ipa: String,
}

fn main() {
    let ipa = Cli::parse().ipa;

    let parsed_ipa = IPA::new(ipa);

    println!("{}", RuPhonemeSec::new(parsed_ipa));
}
