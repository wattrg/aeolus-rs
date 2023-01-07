pub mod config;

use crate::config::AeolusSettings;

fn main() {
    println!("Hello, world!");

    let _settings = AeolusSettings::new().unwrap();
}
