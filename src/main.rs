mod ipa;
mod ru;

use clap::Parser;
use ipa::Ipa;
use ru::Ru;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    ipa: String,
}

fn main() -> Result<(), ipa::SoundError> {
    let ipa = Cli::parse().ipa;

    let parsed_ipa = Ipa::new(&ipa)?;

    println!("{}", Ru::new(parsed_ipa));

    Ok(())
}
