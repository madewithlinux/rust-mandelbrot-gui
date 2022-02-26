use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuncConfig {
    pub path: String,
    #[serde(default = "FuncConfig::default_name")]
    pub name: String,
    #[serde(default)]
    pub options: HashMap<String, String>,
}

impl FuncConfig {
    fn default_name() -> String {
        "default".to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_width")]
    pub width: u32,
    #[serde(default = "Config::default_height")]
    pub height: u32,
    #[serde(default = "Config::default_chunk_size")]
    pub chunk_size: usize,
    pub fractal_config: FuncConfig,
    pub color_config: FuncConfig,
}

impl Config {
    fn default_height() -> u32 {
        1024
    }
    fn default_width() -> u32 {
        1024
    }
    fn default_chunk_size() -> usize {
        32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigManager {
    config_path: Option<PathBuf>,
    config: Config,
    // TODO: track if there are any unsaved changes
}

impl ConfigManager {
    pub fn new(width: u32, height: u32, fractal_lib_path: &str, color_lib_path: &str) -> Self {
        Self {
            config_path: None,
            config: Config {
                width,
                height,
                chunk_size: Config::default_chunk_size(),
                fractal_config: FuncConfig {
                    path: fractal_lib_path.to_owned(),
                    name: FuncConfig::default_name(),
                    options: Default::default(),
                },
                color_config: FuncConfig {
                    path: color_lib_path.to_owned(),
                    name: FuncConfig::default_name(),
                    options: Default::default(),
                },
            },
        }
    }

    pub fn update<F: FnOnce(&mut Config)>(&mut self, mutate: F) {
        (mutate)(&mut self.config)
    }

    pub fn config_path(&self) -> Option<&Path> {
        match &self.config_path {
            Some(p) => Some(p.as_path()),
            None => None,
        }
    }
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn save(&self) -> anyhow::Result<()> {
        match self.config_path() {
            Some(p) => self.write_to_path(p),
            None => Err(anyhow::anyhow!("invalid path, missing parent")),
        }
    }

    pub fn save_as(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.write_to_path(path.as_ref())?;
        self.config_path = Some(path.as_ref().to_owned());
        Ok(())
    }

    fn write_to_path(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let parent_path = path
            .as_ref()
            .parent()
            .ok_or(anyhow::anyhow!("invalid path, missing parent"))?;
        let output_file = NamedTempFile::new_in(parent_path)?;
        serde_yaml::to_writer(&output_file, &self.config)?;
        output_file.persist(path.as_ref())?;
        Ok(())
    }

    pub fn read_from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let input_file = std::fs::File::open(path.as_ref()).context("open file")?;
        let config = serde_yaml::from_reader(input_file).context("deserialize")?;
        Ok(Self {
            config_path: Some(path.as_ref().to_owned()),
            config,
        })
    }
}
