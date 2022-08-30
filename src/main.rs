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
#[clap(
    about = "Mycroft is a tool aimed at helping you monitoring your time.\n\nYou just have to tell Mycroft when you start working on your project with the `start` command, and you can stop the timer when you're done with the `stop` command."
)]
struct Cli {
    #[clap(
        global = true,
        long = "version",
        help = "Show the version and exit.",
        exclusive = true,
        display_order = 9999
    )]
    version_flag: bool,

    #[clap(
        global = true,
        long = "color",
        help = "Color output",
        conflicts_with = "no-color",
        display_order = 9998
    )]
    color: bool,

    #[clap(
        global = true,
        long = "no-color",
        help = "Don't color output",
        conflicts_with = "color",
        display_order = 9997
    )]
    no_color: bool,

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
    if args.version_flag {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

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
