use clap::Parser;
use commands::Commands;

mod log;
mod commands;
mod srcds;

const FOLDER: &'static str = ".gmt";
const SETTINGS_FILE: &'static str = "settings.yml";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

fn main() {
    let cli = Args::parse();

    commands::handle(&cli.command);
}