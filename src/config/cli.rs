use clap::{Command, Arg};

pub fn setup_cli() -> Command {
    Command::new("aeolus")
        .about("A toy CFD code")
        .version("0.1")
        .subcommand_required(true)
        .subcommand(
            Command::new("prep")
                .about("Prepare a simulation")
                .arg(
                    Arg::new("sim")
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("run")
                .about("Run a simulation"),
        )
        .subcommand(
            Command::new("clean")
                .about("Clean out the simulation files"),
        )
        .arg(
            Arg::new("verbosity")
                .help("Set amout of information printed")
                .short('v')
                .long("verbosity")
        )
}
