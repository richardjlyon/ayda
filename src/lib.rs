//! # Ask your Documents Anything
//!
//! Wouldn't it be handy if you could interrogate your documents as though ChatGPT were your personal research assistant?
//!
//! **Now you can.**
//!
//! [Github]()
//!
//! [Documentation]()
//!
//!
//! ## How it works
//!
//! The `aza` command line application is a wrapper around the [AnythingLLM Desktop application](https://useanything.com/). It allows you to interact with your documents in a conversational manner, using the power of ChatGPT to summarise, analyse, and generate content. The model is run locally on your own machine, so you can be sure that your data is secure and private.
//!
//! The application is designed to work with [Zotero](https://www.zotero.org/), a reference management software. You can import your Zotero library into the application, create workspaces for different collections, and interact with the documents in those workspaces using natural language queries.
//!
//! The application is designed to be easy to use and flexible, allowing you to focus on your research and writing without getting bogged down in the technical details. The language model can summarise documents, answer questions, generate content, and more, making it a powerful tool for researchers, writers, and students.
//!
//! ## Usage
//!
//! ```bash
//! > aza import --source zotero climate
//! ...
//! 539 PDFs imported to workspace 'zotero-climate'
//!
//! > aza list
//!
//! WORKSPACES
//! folder-logseq
//! zotero-climate
//! zotero-covid
//! zotero-politics
//!
//! > aza chat zotero-climate
//!
//! Prompt:
//! Summarise Berger 2016 "Interglacials of the last 800,000 years"
//!
//! Berger (2016) discusses the interglacial periods that have occurred during the
//! last 800,000 years. He describes these periods as relatively short-lived
//! intervals between glacial advances, characterized by warmer temperatures and
//! increased biotic productivity. The most recent interglacial period, known as
//! the Holocene, began approximately 11,700 years ago and is currently ongoing.
//! Berger also highlights the importance of Milankovitch cycles in shaping these
//! climate oscillations, specifically changes in Earth’s orbital parameters and
//! axial tilt.
//! ```

extern crate core;

use std::path::PathBuf;

use dialoguer::Input;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};

use crate::app::commands::admin;

pub mod anythingllm;
pub mod app;
pub mod zotero;

/// Configuration parameters for the application.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub zotero_user_id: String,
    pub zotero_api_key: String,
    pub zotero_library_root_path: String,
    pub anythingllm_api_key: String,
    pub anythingllm_ip: String,
    pub anythingllm_port: String,
}

impl Config {
    pub fn from_file() -> eyre::Result<Self> {
        let config_path = Config::get_config_path();
        let file = std::fs::File::open(config_path)?;
        let config: Config = serde_json::from_reader(file)?;

        Ok(config)
    }

    pub fn check_config() -> eyre::Result<PathBuf> {
        let config_path = Config::get_config_path();
        if std::fs::File::open(&config_path).is_err() {
            admin::configure(&config_path).wrap_err("couldn't configure")?;
        }

        Ok(config_path)
    }

    fn get_config_path() -> PathBuf {
        let dirs = directories_next::ProjectDirs::from("com", "richardlyon", "aza").unwrap();
        dirs.config_dir().join("config.json")
    }
}

/// Get configuration parameters from the user.
pub fn get_config_parameters() -> Config {
    let zotero_user_id: String = Input::new()
        .with_prompt("Zotero User ID")
        .interact_text()
        .unwrap();

    let zotero_api_key: String = Input::new()
        .with_prompt("Zotero Api Key")
        .interact_text()
        .unwrap();

    let zotero_library_root_path = Input::new()
        .with_prompt("Zotero Library Root Path")
        .interact_text()
        .unwrap();

    let anythingllm_api_key = Input::new()
        .with_prompt("AnythingLLM Api Key")
        .interact_text()
        .unwrap();

    let anythingllm_ip = Input::new()
        .with_prompt("AnythingLLM IP")
        .interact_text()
        .unwrap();

    let anythingllm_port = Input::new()
        .with_prompt("AnythingLLM Port")
        .interact_text()
        .unwrap();

    Config {
        zotero_user_id,
        zotero_api_key,
        zotero_library_root_path,
        anythingllm_api_key,
        anythingllm_ip,
        anythingllm_port,
    }
}
