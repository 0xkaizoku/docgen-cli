pub mod docx;
pub mod html;
pub mod pdf;
pub mod xlsx;

use crate::cli::{OutputFormat, Theme};
use crate::templates::DocumentMeta;
use anyhow::{anyhow, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ConversionResult {
    pub output_path: String,
    pub bytes_written: usize,
    pub duration_ms: u128,
    pub format: OutputFormat,
}

pub fn convert_document(
    input_text: &str,
    output_path: &Path,
    format: Option<OutputFormat>,
    theme: Theme,
    title: Option<String>,
    author: Option<String>,
) -> Result<ConversionResult> {
    let start_time = std::time::Instant::now();

    let target_format = format
        .or_else(|| OutputFormat::from_path(output_path))
        .ok_or_else(|| {
            anyhow!(
                "Could not determine output format from path '{}'. Please specify --format (pdf|docx|xlsx|html).",
                output_path.display()
            )
        })?;

    let default_title = if output_path.to_string_lossy() == "-" {
        "Document".to_string()
    } else {
        output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Document")
            .to_string()
    };

    let meta = DocumentMeta {
        title: title.unwrap_or(default_title),
        author,
        date: None,
        theme,
    };

    let bytes = match target_format {
        OutputFormat::Html => html::render_html_bytes(input_text, &meta)?,
        OutputFormat::Docx => docx::render_docx_bytes(input_text, &meta)?,
        OutputFormat::Xlsx => xlsx::render_xlsx_bytes(input_text, &meta)?,
        OutputFormat::Pdf => pdf::render_pdf_bytes(input_text, &meta)?,
    };

    if output_path.to_string_lossy() == "-" {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(&bytes)?;
        handle.flush()?;
    } else {
        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(output_path, &bytes)?;
    }

    let elapsed = start_time.elapsed().as_millis();

    Ok(ConversionResult {
        output_path: output_path.to_string_lossy().to_string(),
        bytes_written: bytes.len(),
        duration_ms: elapsed,
        format: target_format,
    })
}
