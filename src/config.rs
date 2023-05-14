use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use crate::options::Options;

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config_path: PathBuf,
}

impl TryFrom<Options> for Config {
    type Error = anyhow::Error;
    fn try_from(value: Options) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config_path = get_config_path(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            config_path,
            pwd,
        });
    }
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let number_of_arguments = value.len() - 1;
        let term = value.get(0).expect("Whooah, man, stepback");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!(
                    "add operation expects 2 arguments but got {}",
                    number_of_arguments
                ));
            }
            let mut drain = value.drain(1..3);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }
        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "remove operation expects 1 argument but got {}",
                    number_of_arguments
                ));
            }
            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }
        if value.len() > 1 {
            return Err(anyhow!(
                "print operation expects 0-1 arguments but got {}",
                value.len()
            ));
        }
        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config_path(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }
    let mut home = if let Ok(home) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(home)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
    } else {
        panic!("Couldn't load home");
    };

    home.push("projector");
    home.push("projector.json");
    return Ok(home);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }
    let pwd = std::env::current_dir().context("unable to get current directory")?;
    let pwd = PathBuf::from(pwd);

    return Ok(pwd);
}
