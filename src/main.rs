#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::Parser;
use std::path::PathBuf;

use chrono::{DateTime, Duration, Utc};
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;

use crate::cmds::Commands;
use crate::config::Config;

mod cmds;
mod config;
mod lifecycle;
mod sorters;

/// Get a path to the config file
fn get_config_path() -> PathBuf {
    let config_file = "config.yaml";

    match dirs::config_dir() {
        Some(mut path) => {
            path.push(config_file);

            return path;
        }
        None => PathBuf::from(format!("/{config_file}")),
    }
}

fn create_format_dispatch(colors: Option<ColoredLevelConfig>) -> Dispatch {
    Dispatch::new().format(move |out, message, record| {
        if let Some(colors) = colors {
            out.finish(format_args!(
                "{} {} [{}] {}",
                Utc::now().format("%Y/%m/%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        } else {
            out.finish(format_args!(
                "{} {} [{}] {}",
                Utc::now().format("%Y/%m/%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        }
    })
}

fn main() {
    let cli = cmds::Cli::parse();

    let logging_colors = ColoredLevelConfig::new()
        .debug(Color::BrightMagenta)
        .info(Color::BrightCyan)
        .warn(Color::BrightYellow)
        .error(Color::BrightRed);

    let mut dispatch = Dispatch::new()
        .chain(create_format_dispatch(Some(logging_colors)).chain(std::io::stdout()));

    dispatch.apply().expect("Couldn't start logger.");

    log::set_max_level(LevelFilter::Debug);

    match &cli.command {
        Some(Commands::Config) => {
            let config_path = get_config_path();
            let config = Config::load(config_path).unwrap();

            println!("{:#?}", config);

            ()
        }
        None => (),
        _ => todo!(),
        // Commands::Lifecycle(cmds) => ,
        // Commands::Sorter(cmds) =>,
    }
}
