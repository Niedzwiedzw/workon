#![feature(command_access)]
#![warn(rust_2018_idioms, future_incompatible)]
#![deny(clippy::all, clippy::if_not_else, clippy::enum_glob_use, clippy::wrong_pub_self_convention)]

pub mod config;
pub mod error;
pub mod terminal;

use config::WorkonConfig;

use clap::{AppSettings, Clap};
use terminal::Alacritty;

use crate::config::ProjectConfig;
use crate::error::WorkonError;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Wojciech Brożek <wojciech.brozek@niedzwiedz.it>"
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// project to run, must be defined in a config beforehand
    project_name: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let config = WorkonConfig::current()?;
    let project: ProjectConfig = match opts.project_name {
        Some(project_name) => config
            .projects
            .into_iter()
            .find(|p| p.project_name == project_name)
            .ok_or(WorkonError::InvalidProjectNameError)?,
        None => {
            let project_names = config
                .projects
                .into_iter()
                .map(|p| p.project_name);
            panic!(
                "you haven't provided a project name, try one of the following:\n{}",
                project_names
                    .map(|name| format!("workon {}", name))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    };
    crate::terminal::startup::<Alacritty>(&project)?;
    Ok(())
}
