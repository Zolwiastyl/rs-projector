use anyhow::Result;
use clap::Parser;
use projector::config::Config;

fn main() -> Result<()> {
    let parsed: Config = projector::options::Options::parse().try_into()?;

    println!("{:?}", parsed);
    return Ok(());
}
