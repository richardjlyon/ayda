//! Administrative commands
//!
//! These commands are used to configure the application and perform other administrative tasks.
//!
//! # Examples
//! > aza admin configure
//!

use std::fs;
use std::io::{Error, Write};
use std::path::PathBuf;

use dialoguer::Editor;
use serde_json::{from_reader, from_str, to_string_pretty};
use tracing::{instrument, trace};

use crate::Config;

#[instrument]
pub fn configure(config_path: &PathBuf) -> Result<(), Error> {
    if !config_path.exists()
        || fs::File::open(config_path)
            .and_then(|f| from_reader::<_, Config>(f).map_err(std::io::Error::from))
            .is_err()
    {
        println!("Config file not found or invalid. Creating a new one.");
        let config = crate::get_config_parameters();
        let config_str = to_string_pretty(&config).unwrap();
        if let Some(parent_dir) = config_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        let mut file = fs::File::create(config_path)?;
        file.write_all(config_str.as_bytes())?;

        trace!("Created config file: {:?}", &file);
        println!("Config file created at {:?}", config_path);

        Ok(())
    } else {
        println!("Editing config file.");
        let file = fs::File::open(config_path)?;
        let config: Config = from_reader(&file)?;
        let config_str = to_string_pretty(&config).unwrap();

        if let Some(rv) = Editor::new().edit(&config_str).unwrap() {
            println!("Your config:");
            println!("{}", rv);

            // round trip to ensure the edited config is valid
            let config: Config = from_str(&rv).unwrap();
            let config_str = to_string_pretty(&config).unwrap();

            let mut file = fs::File::create(config_path)?;
            file.write_all(config_str.as_bytes())?;
        } else {
            println!("Abort!");
        }

        Ok(())
    }
}
