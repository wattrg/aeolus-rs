
use aeolus::cli::{Cli,Commands};
use clap::Parser;

use aeolus::settings::AeolusSettings;
use aeolus::prep::prep_sim;
use common::DynamicResult;

fn main() -> DynamicResult<()> {
    // parse the command line arguments
    let args = Cli::parse(); 

    // set up generic settings
    let settings = AeolusSettings::new(&args)?;

    // perform the sub command requested by the user
    match args.command {
        Commands::Prep{mut prep_file} => {
            prep_sim(&mut prep_file, &settings)?;
        }
        Commands::Run => {
            println!("Running the simulation");
        }
        Commands::Clean => {
            println!("Cleaning the simulation files");
        }
    }
    Ok(())
}
