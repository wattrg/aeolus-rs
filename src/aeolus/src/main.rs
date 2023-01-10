pub mod cli;
pub mod settings;
pub mod prep;

use cli::{Cli,Commands};
use clap::Parser;

use settings::AeolusSettings;
use prep::prep_sim;

fn main() {
    // parse the command line arguments
    let args = Cli::parse(); 

    // set up generic settings
    let settings = AeolusSettings::new(&args).unwrap();

    // perform the sub command requested by the user
    match args.command {
        Commands::Prep{mut prep_file} => {
            prep_sim(&mut prep_file, &settings).unwrap();
        }
        Commands::Run => {
            println!("Running the simulation");
        }
        Commands::Clean => {
            println!("Cleaning the simulation files");
        }
    }
}
