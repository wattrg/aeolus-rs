
use aeolus::cli::{Cli,Commands};
use aeolus::logging::UserLogger;
use clap::Parser;

use aeolus::settings::AeolusSettings;
use aeolus::prep::prep_sim;
use common::DynamicResult;

fn main() -> DynamicResult<()> {
    // parse the command line arguments
    let args = Cli::parse(); 

    // set up generic settings
    let settings = AeolusSettings::new(&args)?;
    let log = UserLogger::with_verbosity(settings.verbosity());

    // perform the sub-command requested by the user
    match args.command {
        Commands::Prep{mut prep_file} => {
            prep_sim(&mut prep_file, &settings)?;
        }
        Commands::Run{start_time_index: _} => {
            println!("Running the simulation");
        }
        Commands::Post => {}
        Commands::Clean => { settings.file_structure().clean(&log)?; }
    }
    Ok(())
}
