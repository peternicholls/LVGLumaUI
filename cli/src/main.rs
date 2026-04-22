//! Operator-facing entry point for LumaUI.
//!
//! All stage orchestration goes through this binary. Parser, semantic, and
//! backend stages stay layer-isolated; this module only sequences them and
//! presents diagnostics and progress to the operator.

use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use lumaui_backend_lvgl_c::{generate_files, GeneratedFile};
use lumaui_compiler::{Diagnostic, ProjectLayout, Severity, WorkspaceConfig, CONFIG_FILE_NAME};
use lumaui_parser::{parse_document, Document};
use lumaui_semantic::{analyze_documents, AnalysisInput};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "lumaui", about = "Luma UI for LVGL — compiler CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a new LumaUI project on disk.
    Init {
        path: PathBuf,
        #[arg(long, default_value = "minimal")]
        name: String,
        #[arg(long, default_value_t = false)]
        force: bool,
    },
    /// Validate the authored source for a project without writing output.
    Validate { project: PathBuf },
    /// Build the project and write generated LVGL C to disk.
    Build { project: PathBuf },
    /// Reserved: launch a preview runner. Currently gated.
    Preview { project: PathBuf },
    /// Print discovery / configuration health for a project.
    Doctor { project: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { path, name, force } => init_project(&path, &name, force),
        Command::Validate { project } => validate_project(&project),
        Command::Build { project } => build_project(&project),
        Command::Preview { project } => preview_project(&project),
        Command::Doctor { project } => doctor_project(&project),
    }
}

// ---------- Init ----------

const STARTER_SCREEN: &str = "<Screen id=\"home\">\n  <Column class=\"root\">\n    <Text id=\"title\" text=\"Hello LumaUI\"/>\n    <Button id=\"openSettings\" onPress=\"open_settings\">\n      <Text text=\"Settings\"/>\n    </Button>\n  </Column>\n</Screen>\n";

const STARTER_STYLE: &str =
    ".root { padding: 16; background-color: #20242b; }\n#title { text-color: #f5f7fa; }\n";

fn init_project(path: &Path, name: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        let mut entries = fs::read_dir(path)
            .with_context(|| format!("reading {}", path.display()))?
            .peekable();
        if entries.peek().is_some() {
            bail!(
                "destination {} is not empty; pass --force to overwrite",
                path.display()
            );
        }
    }
    fs::create_dir_all(path)?;
    let config = WorkspaceConfig::starter(name);
    fs::write(path.join(CONFIG_FILE_NAME), config.to_toml_string()?)?;

    let screens = path.join("ui/screens");
    let styles = path.join("ui/styles");
    fs::create_dir_all(&screens)?;
    fs::create_dir_all(&styles)?;
    fs::write(screens.join("main.lui"), STARTER_SCREEN)?;
    fs::write(styles.join("theme.lus"), STARTER_STYLE)?;

    eprintln!("[init] wrote starter project to {}", path.display());
    Ok(())
}

// ---------- Pipeline ----------

struct Pipeline {
    project_root: PathBuf,
    config: WorkspaceConfig,
    documents: Vec<Document>,
}

fn load_pipeline(project: &Path) -> Result<Pipeline> {
    let config_path = project.join(CONFIG_FILE_NAME);
    let config = WorkspaceConfig::load_from_file(&config_path)?;
    let layout = ProjectLayout::discover(project, &config)?;

    let mut documents = Vec::new();
    let parse_files: Vec<PathBuf> = layout
        .screen_files
        .iter()
        .chain(layout.style_files.iter())
        .cloned()
        .collect();

    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    for file in &parse_files {
        let src =
            fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;
        match parse_document(file, &src) {
            Ok(doc) => documents.push(doc),
            Err(diags) => diagnostics.extend(diags),
        }
    }

    if !diagnostics.is_empty() {
        present_diagnostics(&diagnostics);
        bail!("parse stage reported {} diagnostic(s)", diagnostics.len());
    }

    eprintln!(
        "[parser] {} screen(s), {} style document(s)",
        layout.screen_files.len(),
        layout.style_files.len()
    );

    Ok(Pipeline {
        project_root: project.to_path_buf(),
        config,
        documents,
    })
}

fn validate_project(project: &Path) -> Result<()> {
    let pipeline = load_pipeline(project)?;
    let outcome = analyze_documents(
        AnalysisInput {
            project_name: pipeline.config.project_name.clone(),
            symbol_prefix: pipeline.config.naming.symbol_prefix.clone(),
        },
        &pipeline.documents,
    );

    present_diagnostics(&outcome.diagnostics);
    eprintln!(
        "[semantic] {} diagnostic(s) total",
        outcome.diagnostics.len()
    );

    if outcome.has_errors() {
        bail!("validation failed");
    }
    eprintln!("[validate] OK");
    Ok(())
}

fn build_project(project: &Path) -> Result<()> {
    let pipeline = load_pipeline(project)?;
    let outcome = analyze_documents(
        AnalysisInput {
            project_name: pipeline.config.project_name.clone(),
            symbol_prefix: pipeline.config.naming.symbol_prefix.clone(),
        },
        &pipeline.documents,
    );
    present_diagnostics(&outcome.diagnostics);
    if outcome.has_errors() {
        bail!("validation failed; refusing to write generated files");
    }
    let project_ir = outcome
        .project
        .ok_or_else(|| anyhow!("semantic stage produced no project despite no errors"))?;
    if project_ir.screens.is_empty() {
        bail!("no screens to build");
    }
    eprintln!(
        "[ir] {} screen(s) lowered with prefix `{}`",
        project_ir.screens.len(),
        project_ir.symbol_prefix
    );

    let files = generate_files(&project_ir);
    let output_root = pipeline.config.output_root(&pipeline.project_root);
    write_generated(&output_root, &files)?;
    eprintln!(
        "[backend] wrote {} file(s) to {}",
        files.len(),
        output_root.display()
    );
    eprintln!("[build] OK");
    Ok(())
}

fn write_generated(output_root: &Path, files: &[GeneratedFile]) -> Result<()> {
    fs::create_dir_all(output_root)?;
    for file in files {
        let target = output_root.join(&file.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, &file.contents)
            .with_context(|| format!("writing {}", target.display()))?;
    }
    Ok(())
}

fn preview_project(_project: &Path) -> Result<()> {
    bail!("`preview` is gated until the runtime preview stage is ratified");
}

fn doctor_project(project: &Path) -> Result<()> {
    let config_path = project.join(CONFIG_FILE_NAME);
    if !config_path.exists() {
        bail!("missing {}", config_path.display());
    }
    let config = WorkspaceConfig::load_from_file(&config_path)?;
    let layout = ProjectLayout::discover(project, &config)?;
    eprintln!(
        "[doctor] project_name={} lvgl_version={} symbol_prefix={}",
        config.project_name, config.lvgl_version, config.naming.symbol_prefix
    );
    eprintln!(
        "[doctor] {} screen(s), {} style document(s)",
        layout.screen_files.len(),
        layout.style_files.len()
    );
    for s in &layout.screen_files {
        eprintln!("[doctor]   screen: {}", s.display());
    }
    for s in &layout.style_files {
        eprintln!("[doctor]   style:  {}", s.display());
    }
    Ok(())
}

fn present_diagnostics(diagnostics: &[Diagnostic]) {
    for d in diagnostics {
        let line = d.to_string();
        match d.severity {
            Severity::Error => eprintln!("{line}"),
            Severity::Warning | Severity::Note => eprintln!("{line}"),
        }
    }
}
