#[macro_use]
extern crate serde_derive;

use std::{collections::HashMap, path};

use anyhow::Context;
use clap::{arg, Command};

const APP_NAME: &str = "tpin";

#[derive(Serialize, Deserialize, Default)]
struct Config {
    aliases: HashMap<String, String>,
}

fn cli() -> Command {
    Command::new(env!("CARGO_CRATE_NAME"))
        .subcommand_required(true)
        .subcommands([
            Command::new("new")
                .visible_alias("n")
                .about("Create new/replace existing alias")
                .arg(arg!(<alias> "Alias to create"))
                .arg_required_else_help(true)
                .arg(arg!(<path> "Path to create alias for"))
                .arg_required_else_help(true),
            Command::new("delete")
                .visible_alias("d")
                .about("Delete alias")
                .arg(arg!(<alias> "Alias to delete"))
                .arg_required_else_help(true),
            Command::new("list").alias("l").about("List all aliases"),
            Command::new("print")
                .visible_alias("p")
                .about("Print alias to stdout")
                .arg(arg!(<alias> "Alias to print"))
                .arg_required_else_help(true),
            Command::new("run")
                .visible_alias("r")
                .about("Run alias")
                .arg(arg!(<alias> "Alias to run"))
                .arg_required_else_help(true),
        ])
}

fn main() -> anyhow::Result<()> {
    let config = confy::load::<Config>(APP_NAME, None)?;
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", matches)) => {
            let alias = matches
                .get_one::<String>("alias")
                .context("Alias not provided")?;

            let path = matches
                .get_one::<String>("path")
                .context("Path not provided")?;

            let path = path::absolute(path)?;

            let mut config = config;
            config
                .aliases
                .insert(alias.into(), path.as_path().to_string_lossy().into());
            confy::store(APP_NAME, None, &config)?;
        }
        Some(("delete", matches)) => {
            let alias = matches
                .get_one::<String>("alias")
                .context("Alias not provided")?;

            let mut config = config;
            config.aliases.remove(alias).context("Alias not found")?;
            confy::store(APP_NAME, None, &config)?;
        }
        Some(("list", _)) => {
            for (alias, path) in config.aliases {
                println!("{} -> {}", alias, path);
            }
        }
        Some(("print", matches)) => {
            let alias = matches
                .get_one::<String>("alias")
                .context("Alias not provided")?;

            let path = config.aliases.get(alias).context("Alias not found")?;

            println!("{}", path);
        }
        Some(("run", matches)) => {
            let alias = matches
                .get_one::<String>("alias")
                .context("Alias not provided")?;

            let path = config.aliases.get(alias).context("Alias not found")?;

            let mut command = std::process::Command::new(path);

            let status = command.status().context("Failed to execute command")?;

            std::process::exit(
                status
                    .code()
                    .context("Could not get exit code of child process")?,
            );
        }
        _ => unreachable!(),
    }

    Ok(())
}
