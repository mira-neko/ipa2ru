mod lib;

use clap::Parser;
use lib::{RuPhonemeSeq, Ipa};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    ipa: String,
}

fn main() {
    let ipa = Cli::parse().ipa;

    let parsed_ipa = Ipa::new(ipa);

    println!("{}", RuPhonemeSeq::new(parsed_ipa));
}
