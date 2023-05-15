use anyhow::Result;
use clap::Parser;
use projector::{config::Config, config::Operation, options::Options, projector::Projector};

fn main() -> Result<()> {
    let parsed: Config = Options::parse().try_into()?;

    let mut proj = Projector::from_config(parsed.config_path, parsed.pwd);
    match parsed.operation {
        Operation::Print(None) => {
            let values = proj.get_value_all();
            for (key, value) in values {
                println!("{}: {}", key, value);
            }
        }
        Operation::Print(Some(key)) => {
            let value = proj.get_value(&key);
            match value {
                Some(value) => println!("{}", value),
                None => println!("No value found for key: {}", key),
            }
        }
        Operation::Add(key, value) => {
            proj.set_value(key, value);
            proj.save()?;
        }
        Operation::Remove(key) => {
            proj.remove_value(&key);
            proj.save()?;
        }
    }

    return Ok(());
}
