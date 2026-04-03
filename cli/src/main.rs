use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use lumaui_backend_lvgl_c::generate_files;
use lumaui_compiler::{Diagnostic, ProjectLayout, WorkspaceConfig, CONFIG_FILE_NAME};
use lumaui_parser::{parse_document, Document, DocumentKind};
use lumaui_semantic::analyze_documents;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "lumaui")]
#[command(about = "Compiler tooling for the LumaUI project")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        force: bool,
    },
    Validate {
        #[arg(default_value = ".")]
        project: PathBuf,
    },
    Build {
        #[arg(default_value = ".")]
        project: PathBuf,
    },
    Preview {
        #[arg(default_value = ".")]
        project: PathBuf,
    },
    Doctor {
        #[arg(default_value = ".")]
        project: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { path, name, force } => init_project(&path, name.as_deref(), force),
        Command::Validate { project } => validate_project(&project),
        Command::Build { project } => build_project(&project),
        Command::Preview { project } => preview_project(&project),
        Command::Doctor { project } => doctor_project(&project),
    }
}

fn init_project(path: &Path, name: Option<&str>, force: bool) -> Result<()> {
    fs::create_dir_all(path).with_context(|| format!("failed to create {}", path.display()))?;

    let project_name = name.unwrap_or("lumaui-app");
    let config_path = path.join(CONFIG_FILE_NAME);

    if config_path.exists() && !force {
        bail!(
            "{} already exists; re-run with --force to overwrite starter files",
            config_path.display()
        );
    }

    let config = WorkspaceConfig::starter(project_name);
    let screen_dir = path.join("ui/screens");
    let style_dir = path.join("ui/styles");

    fs::create_dir_all(&screen_dir)
        .with_context(|| format!("failed to create {}", screen_dir.display()))?;
    fs::create_dir_all(&style_dir)
        .with_context(|| format!("failed to create {}", style_dir.display()))?;

    fs::write(&config_path, config.to_toml_string()?)
        .with_context(|| format!("failed to write {}", config_path.display()))?;
    fs::write(screen_dir.join("main.lui"), starter_screen(project_name))
        .with_context(|| format!("failed to write starter screen in {}", screen_dir.display()))?;
    fs::write(style_dir.join("theme.lus"), starter_style())
        .with_context(|| format!("failed to write starter style in {}", style_dir.display()))?;

    println!("initialized starter LumaUI project at {}", path.display());
    Ok(())
}

fn validate_project(path: &Path) -> Result<()> {
    let (project_root, config, layout) = load_project(path)?;
    let documents = parse_project_sources(&layout)?;
    let analysis = analyze_documents(&config.project_name, &documents);

    println!("project: {}", config.project_name);
    println!("root: {}", project_root.display());
    println!("screens: {}", layout.screen_files.len());
    println!("styles: {}", layout.style_files.len());

    for document in &documents {
        println!("validated provisional frontend for {}", document.source_name);
    }

    for diagnostic in analysis.diagnostics {
        println!("{diagnostic}");
    }

    println!("validation completed for first-pass compiler scaffolding");
    Ok(())
}

fn build_project(path: &Path) -> Result<()> {
    let (_, config, layout) = load_project(path)?;
    let documents = parse_project_sources(&layout)?;
    let analysis = analyze_documents(&config.project_name, &documents);

    if analysis.project.screens.is_empty() {
        bail!("build is not available yet; parser and semantic lowering are still being implemented");
    }

    let files = generate_files(&analysis.project);
    println!("planned {} generated files", files.len());
    Ok(())
}

fn preview_project(_path: &Path) -> Result<()> {
    bail!("preview is planned for a later milestone once generated-output flow is in place")
}

fn doctor_project(path: &Path) -> Result<()> {
    let (project_root, config, layout) = load_project(path)?;

    println!("LumaUI doctor report");
    println!("project root: {}", project_root.display());
    println!("config: {}", project_root.join(CONFIG_FILE_NAME).display());
    println!("LVGL baseline: {}", config.lvgl_version);
    println!("source root: {}", layout.source_root.display());
    println!("output root: {}", layout.output_root.display());
    println!("screen files discovered: {}", layout.screen_files.len());
    println!("style files discovered: {}", layout.style_files.len());

    if layout.screen_files.is_empty() {
        println!(
            "warning: no .lui files found under {}",
            layout.source_root.join("screens").display()
        );
    }

    if layout.style_files.is_empty() {
        println!(
            "warning: no .lus files found under {}",
            layout.source_root.join("styles").display()
        );
    }

    Ok(())
}

fn load_project(path: &Path) -> Result<(PathBuf, WorkspaceConfig, ProjectLayout)> {
    let project_root = if path.is_file() {
        path.parent()
            .map(Path::to_path_buf)
            .context("config path must have a parent directory")?
    } else {
        path.to_path_buf()
    };

    let config_path = if path.is_file() {
        path.to_path_buf()
    } else {
        project_root.join(CONFIG_FILE_NAME)
    };

    let config = WorkspaceConfig::load_from_file(&config_path)?;
    let layout = ProjectLayout::discover(&project_root, &config)?;

    Ok((project_root, config, layout))
}

fn parse_project_sources(layout: &ProjectLayout) -> Result<Vec<Document>> {
    let mut documents = Vec::new();

    for path in &layout.screen_files {
        let source =
            fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
        let outcome = parse_document(path, &source, DocumentKind::Markup)
            .map_err(render_parse_errors)?;

        for diagnostic in outcome.diagnostics {
            println!("{diagnostic}");
        }

        println!("tokenized {} tokens from {}", outcome.token_count, path.display());
        documents.push(outcome.document);
    }

    for path in &layout.style_files {
        let source =
            fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
        let outcome = parse_document(path, &source, DocumentKind::Style)
            .map_err(render_parse_errors)?;

        for diagnostic in outcome.diagnostics {
            println!("{diagnostic}");
        }

        println!("tokenized {} tokens from {}", outcome.token_count, path.display());
        documents.push(outcome.document);
    }

    Ok(documents)
}

fn render_parse_errors(errors: Vec<Diagnostic>) -> anyhow::Error {
    let rendered = errors
        .into_iter()
        .map(|error| error.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    anyhow::anyhow!(rendered)
}

fn starter_screen(project_name: &str) -> String {
    format!(
        "<Screen id=\"home\">\n  <Column class=\"root\">\n    <Text text=\"{project_name}\"/>\n    <Button id=\"primaryAction\">\n      <Text text=\"Continue\"/>\n    </Button>\n  </Column>\n</Screen>\n"
    )
}

fn starter_style() -> &'static str {
    ".root {\n  padding: 16;\n}\n"
}
