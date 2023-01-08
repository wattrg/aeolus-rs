
use aeolus_rs::config::{settings::AeolusSettings, cli::{Cli,Commands}};
use clap::Parser;

fn main() {
    // parse the command line arguments
    let args = Cli::parse(); 

    // set up generic settings
    let settings = AeolusSettings::new(&args).unwrap();

    println!("verbosity = {}", settings.verbosity());

    // perform the sub command requested by the user
    match args.command {
        Commands::Prep{prep_file} => {
            println!("Preparing a simulation with {:?}", prep_file);
        }
        Commands::Run => {
            println!("Running the simulation");
        }
        Commands::Clean => {
            println!("Cleaning the simulation files");
        }
    }
}
