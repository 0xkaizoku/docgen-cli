# 📚 `docgen-cli` Rust Crate & Library Guide

`docgen-cli` is designed to be used both as a command-line binary and as a high-performance, zero-dependency Rust crate (`docgen_cli`) in backend services, web applications, background workers, and automation tools.

---

## 📦 Adding `docgen-cli` to Your Project

Add `docgen-cli` to your `Cargo.toml`:

```toml
[dependencies]
docgen-cli = { git = "https://github.com/0xkaizoku/docgen-cli" }
```

---

## 🚀 Quick Examples

### 1. High-Level Conversion (`convert_document`)

```rust
use docgen_cli::{convert_document, Theme, OutputFormat};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let markdown = "# Monthly Performance\n\nRevenue grew by **42%** this quarter.";

    let result = convert_document(
        markdown,
        Path::new("report.docx"),
        Some(OutputFormat::Docx),
        Theme::ModernExecutive,
        Some("Monthly Performance".to_string()),
        Some("Engineering Lead".to_string()),
    )?;

    println!(
        "Rendered {} ({} bytes) in {}ms",
        result.output_path, result.bytes_written, result.duration_ms
    );
    Ok(())
}
```

---

### 2. Direct In-Memory Byte Generation

Generate raw document bytes (`Vec<u8>`) directly in memory without writing to the disk:

```rust
use docgen_cli::{
    convert_markdown_to_docx,
    convert_markdown_to_html,
    convert_to_xlsx,
    DocumentMeta,
    Theme,
};

fn generate_in_memory() -> anyhow::Result<()> {
    let meta = DocumentMeta {
        title: "Executive Memo".to_string(),
        author: Some("CTO".to_string()),
        date: Some("2026-08-17".to_string()),
        theme: Theme::TechSpec,
    };

    // 1. Generate DOCX bytes
    let docx_bytes: Vec<u8> = convert_markdown_to_docx("# Tech Spec\n\nDetails...", &meta)?;

    // 2. Generate HTML string
    let html_string: String = convert_markdown_to_html("# Preview\n\nContent...", &meta)?;

    // 3. Generate Excel XLSX bytes from JSON
    let json_data = r#"[{"Metric": "QPS", "Value": 14000}]"#;
    let xlsx_bytes: Vec<u8> = convert_to_xlsx(json_data, &meta)?;

    Ok(())
}
```

---

## 🌐 Web Server Integration (Axum Example)

```rust
use axum::{
    body::Bytes,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use docgen_cli::{convert_markdown_to_docx, DocumentMeta, Theme};

async fn generate_docx_handler(body: String) -> Result<impl IntoResponse, StatusCode> {
    let meta = DocumentMeta {
        title: "API Generated Document".to_string(),
        author: Some("Cloud Service".to_string()),
        date: None,
        theme: Theme::ModernExecutive,
    };

    match convert_markdown_to_docx(&body, &meta) {
        Ok(docx_bytes) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                    .parse()
                    .unwrap(),
            );
            headers.insert(
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"document.docx\"".parse().unwrap(),
            );
            Ok((headers, Bytes::from(docx_bytes)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/render/docx", post(generate_docx_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```
