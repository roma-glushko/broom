use clap::{Parser, Subcommand};

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
