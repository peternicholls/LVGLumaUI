use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE_NAME: &str = "lumaui.toml";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceConfig {
    pub project_name: String,
    pub lvgl_version: String,
    #[serde(default = "default_source_dir")]
    pub source_dir: PathBuf,
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,
    #[serde(default)]
    pub naming: NamingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NamingConfig {
    #[serde(default = "default_symbol_prefix")]
    pub symbol_prefix: String,
}

impl Default for NamingConfig {
    fn default() -> Self {
        Self {
            symbol_prefix: default_symbol_prefix(),
        }
    }
}

impl WorkspaceConfig {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read config file {}", path.display()))?;

        toml::from_str(&raw)
            .with_context(|| format!("failed to parse config file {}", path.display()))
    }

    pub fn starter(project_name: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
            lvgl_version: "9.x".to_string(),
            source_dir: default_source_dir(),
            output_dir: default_output_dir(),
            naming: NamingConfig::default(),
        }
    }

    pub fn to_toml_string(&self) -> Result<String> {
        toml::to_string_pretty(self).context("failed to serialize workspace config")
    }

    pub fn source_root(&self, project_root: &Path) -> PathBuf {
        project_root.join(&self.source_dir)
    }

    pub fn output_root(&self, project_root: &Path) -> PathBuf {
        project_root.join(&self.output_dir)
    }
}

fn default_source_dir() -> PathBuf {
    PathBuf::from("ui")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("generated/ui")
}

fn default_symbol_prefix() -> String {
    "lumaui_".to_string()
}
