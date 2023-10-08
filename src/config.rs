use std::path::PathBuf;
use std::time::SystemTime;
use std::{fs, io};

use log::{debug, info};

use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

/// Generate an auth token to talk to sorters
fn generate_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 32)
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LifecycleRule {
    name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SorterRule {
    name: String,
}

/// broom config that is persisted as a YAML file in the home directory
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub auth_token: String,
    pub lifecycle: Vec<LifecycleRule>,
    pub sorters: Vec<SorterRule>,
}

impl Config {
    /// Load the config file
    pub fn load(config_path: PathBuf) -> io::Result<Self> {
        debug!("Loading Config: {}", config_path.display());

        if !config_path.exists() {
            let config = Self::default();
            config.save(&config_path)?;

            return Ok(config);
        }

        let raw_config = fs::read_to_string(config_path)?;
        let config =
            serde_yaml::from_str::<Config>(&raw_config).expect("Failed to deserialize config");

        return Ok(config);
    }

    pub fn lifecycle_rules(&self) -> &Vec<LifecycleRule> {
        return &self.lifecycle;
    }

    pub fn sorters_rules(&self) -> &Vec<SorterRule> {
        return &self.sorters;
    }

    pub fn save(&self, config_path: &PathBuf) -> io::Result<()> {
        debug!("Saving Config: {:?} {}", &self, config_path.display());

        let raw_config = serde_yaml::to_string(&self).expect("Failed to serialize config");

        if let Some(parent_dir) = config_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        fs::write(config_path, raw_config)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auth_token: generate_token(),
            lifecycle: Vec::new(),
            sorters: Vec::new(),
        }
    }
}
