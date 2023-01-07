pub mod config;

use crate::config::{settings::AeolusSettings, cli::setup_cli};

fn main() {
    // parse the command line arguments
    let cli = setup_cli().get_matches(); 

    // set up generic settings
    let _settings = AeolusSettings::new(&cli).unwrap();

    // perform the sub command requested by the user
    match cli.subcommand() {
        Some(("prep", prep_matches)) => {
            println!("Prepping simulation with {:?}", prep_matches.get_one::<String>("sim"));
        }
        Some(("run", _run_mathces)) => {
            println!("Running the simulation");
        }
        Some(("clean", _clean_matches)) => {
            println!("Cleaning the simulation files");
        }
        _ => unreachable!(),
    }
}
