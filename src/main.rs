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
    println!("{}", Ru::from(Cli::parse().ipa.as_str()));
    Ok(())
}
