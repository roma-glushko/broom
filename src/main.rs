use clap::Parser;

pub mod cmd;

fn main() {
    let cli = cmd::Cli::parse();

    match &cli.command {
        None => {}
        _ => todo!(),
    }
}
