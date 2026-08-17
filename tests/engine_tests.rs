use docgen_cli::engine::convert_document;
use docgen_cli::templates::DocumentMeta;
use docgen_cli::{
    convert_markdown_to_docx, convert_markdown_to_html, convert_to_xlsx, OutputFormat, Theme,
};
use std::fs;

#[test]
fn test_html_rendering_with_all_themes() {
    let markdown = r#"
# Project Alpha Report

This is a **bold statement** and an *italic note*.

- First bullet
- Second bullet

> Important quote from the team.

```rust
fn main() {
    println!("Hello docgen-cli!");
}
```

| Feature | Status | Score |
| :--- | :--- | :--- |
| Speed | Fast | 99 |
| Memory | Low | 100 |
"#;

    let themes = [
        Theme::ModernExecutive,
        Theme::TechSpec,
        Theme::MinimalPaper,
        Theme::CorporateSlate,
        Theme::DarkGlass,
    ];

    for theme in themes {
        let meta = DocumentMeta {
            title: "Test Report".to_string(),
            author: Some("Test Author".to_string()),
            date: Some("2026-08-17".to_string()),
            theme,
        };

        let html =
            convert_markdown_to_html(markdown, &meta).expect("HTML rendering should succeed");

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Report"));
        assert!(html.contains("Test Author"));
        assert!(html.contains("Project Alpha Report"));
        assert!(html.contains("Important quote"));
        assert!(html.contains("<table"));
        assert!(html.contains("<pre") || html.contains("<code"));
    }
}

#[test]
fn test_docx_rendering() {
    let markdown = r#"
# Architecture Overview

This document describes the high level architecture of our system.

## Components

- **Gateway**: Routes API traffic
- **Worker**: Processes background document rendering
- **Storage**: Retains generated artifacts

### Data Table

| Metric | Target | Actual |
| :--- | :--- | :--- |
| Latency | <10ms | 2ms |
| Memory | <50MB | 12MB |

```rust
let config = EngineConfig::default();
```
"#;

    let meta = DocumentMeta {
        title: "Architecture Document".to_string(),
        author: Some("Lead Architect".to_string()),
        date: Some("2026-08-17".to_string()),
        theme: Theme::ModernExecutive,
    };

    let docx_bytes =
        convert_markdown_to_docx(markdown, &meta).expect("DOCX conversion should succeed");

    assert!(!docx_bytes.is_empty());
    // DOCX is a ZIP container: verify magic header [0x50, 0x4B, 0x03, 0x04]
    assert_eq!(&docx_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);
    assert!(docx_bytes.len() > 1000);
}

#[test]
fn test_xlsx_rendering_from_json_objects() {
    let json_data = r#"[
        { "ID": 101, "Name": "Alice", "Role": "Engineer", "Salary": 125000, "Active": true },
        { "ID": 102, "Name": "Bob", "Role": "Designer", "Salary": 115000, "Active": true },
        { "ID": 103, "Name": "Charlie", "Role": "Manager", "Salary": 140000, "Active": false }
    ]"#;

    let meta = DocumentMeta::default();
    let xlsx_bytes =
        convert_to_xlsx(json_data, &meta).expect("XLSX from JSON objects should succeed");

    assert!(!xlsx_bytes.is_empty());
    assert_eq!(&xlsx_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);
    assert!(xlsx_bytes.len() > 1000);
}

#[test]
fn test_xlsx_rendering_from_json_arrays() {
    let json_arrays = r#"[
        ["Department", "Headcount", "Budget"],
        ["Engineering", 42, 5000000],
        ["Product", 15, 2000000],
        ["Operations", 8, 800000]
    ]"#;

    let meta = DocumentMeta::default();
    let xlsx_bytes =
        convert_to_xlsx(json_arrays, &meta).expect("XLSX from JSON arrays should succeed");

    assert!(!xlsx_bytes.is_empty());
    assert_eq!(&xlsx_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);
}

#[test]
fn test_xlsx_rendering_from_markdown_table() {
    let md_table = r#"
| Service | Region | Latency (ms) | Uptime |
| :--- | :--- | :--- | :--- |
| API Gateway | us-east-1 | 12 | 99.99% |
| Auth Service | us-west-2 | 18 | 99.95% |
| Database | eu-central-1 | 24 | 99.999% |
"#;

    let meta = DocumentMeta::default();
    let xlsx_bytes =
        convert_to_xlsx(md_table, &meta).expect("XLSX from markdown table should succeed");

    assert!(!xlsx_bytes.is_empty());
    assert_eq!(&xlsx_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);
}

#[test]
fn test_xlsx_rendering_from_csv() {
    let csv_data = r#"Product,Units Sold,Price,InStock
Widget Pro,150,29.99,true
Gizmo Max,80,49.50,false
Thingamajig,320,12.00,true"#;

    let meta = DocumentMeta::default();
    let xlsx_bytes = convert_to_xlsx(csv_data, &meta).expect("XLSX from CSV should succeed");

    assert!(!xlsx_bytes.is_empty());
    assert_eq!(&xlsx_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);
}

#[test]
fn test_convert_document_to_files() {
    let temp_dir = std::env::temp_dir().join("docgen_cli_test_outputs");
    fs::create_dir_all(&temp_dir).unwrap();

    let md_input = "# Automated Test\n\nTesting document pipeline.";
    let json_input = r#"[{"Task": "Test 1", "Passed": true}]"#;

    // Test HTML file creation
    let html_path = temp_dir.join("test.html");
    let res_html = convert_document(
        md_input,
        &html_path,
        Some(OutputFormat::Html),
        Theme::DarkGlass,
        Some("Test Doc".to_string()),
        None,
    )
    .expect("convert_document HTML should succeed");
    assert_eq!(res_html.format, OutputFormat::Html);
    assert!(html_path.exists());

    // Test DOCX file creation
    let docx_path = temp_dir.join("test.docx");
    let res_docx = convert_document(
        md_input,
        &docx_path,
        Some(OutputFormat::Docx),
        Theme::ModernExecutive,
        None,
        None,
    )
    .expect("convert_document DOCX should succeed");
    assert_eq!(res_docx.format, OutputFormat::Docx);
    assert!(docx_path.exists());

    // Test XLSX file creation
    let xlsx_path = temp_dir.join("test.xlsx");
    let res_xlsx = convert_document(
        json_input,
        &xlsx_path,
        Some(OutputFormat::Xlsx),
        Theme::CorporateSlate,
        None,
        None,
    )
    .expect("convert_document XLSX should succeed");
    assert_eq!(res_xlsx.format, OutputFormat::Xlsx);
    assert!(xlsx_path.exists());

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}
