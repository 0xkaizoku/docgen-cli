//! # `docgen-cli`
//!
//! Blazing-fast zero-dependency universal document engine, CLI tool, and MCP server.
//!
//! `docgen-cli` converts Markdown, JSON, CSV, TSV, text, and terminal command outputs into styled
//! **PDF**, **DOCX**, **XLSX**, and **HTML** documents in milliseconds.
//!
//! ## Example (Rust Library Usage)
//!
//! ```rust,no_run
//! use docgen_cli::{convert_document, Theme, OutputFormat};
//! use std::path::Path;
//!
//! let markdown = "# Hello World\nThis is a report generated with docgen-cli.";
//! let res = convert_document(
//!     markdown,
//!     Path::new("output.docx"),
//!     Some(OutputFormat::Docx),
//!     Theme::ModernExecutive,
//!     Some("My Report".to_string()),
//!     Some("Author Name".to_string()),
//! ).unwrap();
//!
//! println!("Created {} in {}ms", res.output_path, res.duration_ms);
//! ```

pub mod cli;
pub mod engine;
pub mod installer;
pub mod mcp;
pub mod templates;

pub use cli::{Cli, Commands, OutputFormat, Theme};
pub use engine::{convert_document, ConversionResult};
pub use templates::DocumentMeta;

use anyhow::{anyhow, Result};
use clap::Parser;
use installer::init_ai_configurations;
use mcp::run_mcp_server;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Options for CLI conversion execution
#[derive(Debug, Clone, Default)]
pub struct ConversionOptions {
    pub theme: Theme,
    pub format: Option<OutputFormat>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub json: bool,
    pub quiet: bool,
}

/// Main entry point for executing the CLI application
pub async fn run_cli() -> Result<()> {
    let args = Cli::parse();

    if let Some(cmd) = args.command {
        match cmd {
            Commands::Convert {
                input,
                text,
                output,
                theme,
                format,
                title,
                author,
            } => {
                let opts = ConversionOptions {
                    theme,
                    format,
                    title,
                    author,
                    json: args.json,
                    quiet: args.quiet,
                };
                execute_conversion(input.as_ref(), text.as_deref(), &output, opts)?;
            }
            Commands::Mcp => {
                run_mcp_server().await?;
            }
            Commands::InitAi { global } => {
                init_ai_configurations(global)?;
            }
            Commands::ListTemplates => {
                println!("🎨 Built-in Design Themes in docgen-cli:\n");
                let themes = [
                    Theme::ModernExecutive,
                    Theme::TechSpec,
                    Theme::MinimalPaper,
                    Theme::CorporateSlate,
                    Theme::DarkGlass,
                ];
                for (i, t) in themes.iter().enumerate() {
                    println!("  {}. {:<18} - {}", i + 1, t.name(), t.description());
                }
            }
        }
        return Ok(());
    }

    // Direct mode fallback (e.g. `docgen-cli -i input.md -o output.pdf` or `docgen-cli --text "# Title" -o output.docx`)
    if let Some(output_path) = args.output {
        let opts = ConversionOptions {
            theme: args.theme,
            format: args.format,
            title: args.title,
            author: args.author,
            json: args.json,
            quiet: args.quiet,
        };
        execute_conversion(
            args.input.as_ref(),
            args.text.as_deref(),
            &output_path,
            opts,
        )?;
    } else {
        println!("⚡ docgen-cli v0.1.0 - Universal Document Engine & MCP Server");
        println!("Run 'docgen-cli --help' for commands or 'docgen-cli convert <input> -o <output>' to convert.");
    }

    Ok(())
}

fn execute_conversion(
    input_path: Option<&PathBuf>,
    direct_text: Option<&str>,
    output_path: &Path,
    opts: ConversionOptions,
) -> Result<()> {
    let input_text = if let Some(t) = direct_text {
        t.to_string()
    } else {
        let path = input_path
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "-".to_string());
        if path == "-" {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        } else {
            std::fs::read_to_string(&path)
                .map_err(|e| anyhow!("Failed to read input file '{}': {}", path, e))?
        }
    };

    let res = convert_document(
        &input_text,
        output_path,
        opts.format,
        opts.theme,
        opts.title,
        opts.author,
    )?;

    if opts.json {
        println!(
            "{}",
            serde_json::json!({
                "status": "success",
                "output_path": res.output_path,
                "bytes_written": res.bytes_written,
                "duration_ms": res.duration_ms,
                "format": format!("{:?}", res.format).to_lowercase(),
            })
        );
    } else if !opts.quiet && output_path.to_string_lossy() != "-" {
        println!(
            "⚡ Generated {:?} document at '{}' ({} KB) in {}ms",
            res.format,
            res.output_path,
            (res.bytes_written as f64 / 1024.0).ceil() as u64,
            res.duration_ms
        );
    }

    Ok(())
}

/// Direct helper to convert Markdown into styled HTML string
pub fn convert_markdown_to_html(markdown: &str, meta: &DocumentMeta) -> Result<String> {
    engine::html::render_html_string(markdown, meta)
}

/// Direct helper to convert Markdown into DOCX (Word) bytes
pub fn convert_markdown_to_docx(markdown: &str, meta: &DocumentMeta) -> Result<Vec<u8>> {
    engine::docx::render_docx_bytes(markdown, meta)
}

/// Direct helper to convert Markdown into PDF bytes
pub fn convert_markdown_to_pdf(markdown: &str, meta: &DocumentMeta) -> Result<Vec<u8>> {
    engine::pdf::render_pdf_bytes(markdown, meta)
}

/// Direct helper to convert JSON/CSV/Table text into XLSX (Excel) bytes
pub fn convert_to_xlsx(input: &str, meta: &DocumentMeta) -> Result<Vec<u8>> {
    engine::xlsx::render_xlsx_bytes(input, meta)
}
