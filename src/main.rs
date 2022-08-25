mod ru;

use clap::Parser;
use ru::Ru;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    ipa: String,
}

fn main() -> Result<(), ipa_sounds::Error> {
    Ru::try_from(Cli::parse().ipa).map(|ru| println!("{}", ru))
}
