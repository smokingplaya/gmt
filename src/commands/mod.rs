use clap::Subcommand;

pub mod build;
pub mod init;
pub mod start;

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Build,
    Init
}

pub fn handle(command: &Option<Commands>) {
    match command {
        Some(Commands::Build) => build::on_execute(),
        Some(Commands::Init) => init::on_execute(),
        Some(Commands::Start) => start::on_execute(),
        None => return
    }
}