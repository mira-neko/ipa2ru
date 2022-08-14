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

fn main() -> Result<(), ipa::Error> {
    let russian = Some(&Cli::parse().ipa)
        .map(String::as_str)
        .map(Ipa::new)
        .unwrap()
        .map(Ru::new)?;
    println!("{}", russian);
    Ok(())
}
