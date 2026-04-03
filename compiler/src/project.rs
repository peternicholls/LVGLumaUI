use crate::config::WorkspaceConfig;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectLayout {
    pub project_root: PathBuf,
    pub source_root: PathBuf,
    pub output_root: PathBuf,
    pub screen_files: Vec<PathBuf>,
    pub style_files: Vec<PathBuf>,
}

impl ProjectLayout {
    pub fn discover(project_root: &Path, config: &WorkspaceConfig) -> Result<Self> {
        let source_root = config.source_root(project_root);
        let output_root = config.output_root(project_root);
        let screen_files = collect_files(&source_root.join("screens"), "lui")?;
        let style_files = collect_files(&source_root.join("styles"), "lus")?;

        Ok(Self {
            project_root: project_root.to_path_buf(),
            source_root,
            output_root,
            screen_files,
            style_files,
        })
    }
}

fn collect_files(dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();

    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry.with_context(|| format!("failed to inspect {}", dir.display()))?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
            files.push(path);
        }
    }

    files.sort();
    Ok(files)
}
