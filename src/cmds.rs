use clap::builder::TypedValueParser as _;
use clap::{Parser, Subcommand};
use log::LevelFilter;

use crate::lifecycle::cmds::LifecycleCmds;
use crate::sorters::cmds::SorterCmds;

const BANNER: &'static str = "

    ██████╗░██████╗░░█████╗░░█████╗░███╗░░░███╗
    ██╔══██╗██╔══██╗██╔══██╗██╔══██╗████╗░████║
    ██████╦╝██████╔╝██║░░██║██║░░██║██╔████╔██║
    ██╔══██╗██╔══██╗██║░░██║██║░░██║██║╚██╔╝██║
    ██████╦╝██║░░██║╚█████╔╝╚█████╔╝██║░╚═╝░██║
    ╚═════╝░╚═╝░░╚═╝░╚════╝░░╚════╝░╚═╝░░░░░╚═╝

🧹 Sort and clean your files like screenshots on your desktop your way";

#[derive(Parser)]
#[command(author, author, version)]
#[command(about = BANNER)]
pub struct Cli {
    /// Verbosity
    #[arg(
        short,
        long,
        default_value_t = LevelFilter::Info,
        value_parser = clap::builder::PossibleValuesParser::new(["TRACE", "DEBUG", "INFO", "WARN", "ERROR"])
            .map(|s| s.parse::<LevelFilter>().unwrap()),
    )]
    pub verbosity: LevelFilter,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// View the current config
    Config,

    /// Manage lifecycle rules
    Lifecycle(LifecycleCmds),

    /// Sort files you create in real time
    Sorter(SorterCmds),
}
