extern crate diesel;

use clap::Parser;

use anyhow::Result;
use commands::add::AddSubcommand;
use commands::log::LogSubcommand;
use commands::start::StartSubcommand;
use commands::status::StatusSubcommand;
use commands::stop::StopSubcommand;
use commands::MyCommand;
pub mod commands;
mod config;

#[derive(Debug, Parser)]
#[clap(name = "Mycroft")]
#[clap(about = "Watson is a tool aimed at helping you monitoring your time.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Add(AddSubcommand),
    Start(StartSubcommand),
    Stop(StopSubcommand),
    Status(StatusSubcommand),
    Log(LogSubcommand),
}

fn main() -> Result<()> {
    let args = Cli::parse();
    return match args.command {
        Commands::Add(command) => run(&command),
        Commands::Start(command) => run(&command),
        Commands::Stop(command) => run(&command),
        Commands::Status(command) => run(&command),
        Commands::Log(command) => run(&command),
    };
}

fn run(my_command: &dyn MyCommand) -> Result<()> {
    return my_command.run();
}
