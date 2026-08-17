use crate::cli::Theme;
use crate::engine::convert_document;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

pub async fn run_mcp_server() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    for line in stdin.lock().lines() {
        let line_str = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line_str.trim().is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(&line_str) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": format!("Parse error: {}", e) }
                });
                writeln!(stdout_lock, "{}", err_resp)?;
                stdout_lock.flush()?;
                continue;
            }
        };

        let response = handle_mcp_request(&req);
        let resp_json = serde_json::to_string(&response)?;
        writeln!(stdout_lock, "{}", resp_json)?;
        stdout_lock.flush()?;
    }

    Ok(())
}

fn handle_mcp_request(req: &JsonRpcRequest) -> JsonRpcResponse {
    let req_id = req.id.clone().unwrap_or(Value::Null);

    match req.method.as_str() {
        "initialize" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req_id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "docgen-cli-mcp",
                    "version": "0.1.0"
                }
            })),
            error: None,
        },
        "notifications/initialized" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req_id,
            result: Some(json!({})),
            error: None,
        },
        "ping" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req_id,
            result: Some(json!({})),
            error: None,
        },
        "tools/list" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req_id,
            result: Some(json!({
                "tools": [
                    {
                        "name": "doc_convert",
                        "description": "Convert Markdown, JSON, CSV, or text data into PDF, DOCX, XLSX, or HTML documents in milliseconds.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "input_content": { "type": "string", "description": "Markdown text or JSON array data to convert" },
                                "output_path": { "type": "string", "description": "Target output file path (e.g. output.pdf, data.xlsx, report.docx)" },
                                "theme": { "type": "string", "description": "Design theme: modern-executive, tech-spec, minimal-paper, corporate-slate, dark-glass", "default": "modern-executive" },
                                "title": { "type": "string", "description": "Document title" },
                                "author": { "type": "string", "description": "Author name" }
                            },
                            "required": ["input_content", "output_path"]
                        }
                    },
                    {
                        "name": "doc_list_themes",
                        "description": "List all available design themes and document templates in docgen-cli.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    }
                ]
            })),
            error: None,
        },
        "tools/call" => {
            let params = req.params.clone().unwrap_or(Value::Null);
            let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));

            let tool_res = match tool_name {
                "doc_convert" => handle_doc_convert_tool(&args),
                "doc_list_themes" => handle_list_themes_tool(),
                _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
            };

            match tool_res {
                Ok(content_val) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req_id,
                    result: Some(json!({ "content": [{ "type": "text", "text": content_val }] })),
                    error: None,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req_id,
                    result: Some(
                        json!({ "content": [{ "type": "text", "text": format!("Error: {}", e) }], "isError": true }),
                    ),
                    error: None,
                },
            }
        }
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req_id,
            result: None,
            error: Some(json!({ "code": -32601, "message": "Method not found" })),
        },
    }
}

fn handle_doc_convert_tool(args: &Value) -> Result<String> {
    let input_content = args
        .get("input_content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing input_content argument"))?;

    let output_path_str = args
        .get("output_path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing output_path argument"))?;

    let theme_str = args
        .get("theme")
        .and_then(|v| v.as_str())
        .unwrap_or("modern-executive");

    let theme = match theme_str {
        "tech-spec" => Theme::TechSpec,
        "minimal-paper" => Theme::MinimalPaper,
        "corporate-slate" => Theme::CorporateSlate,
        "dark-glass" => Theme::DarkGlass,
        _ => Theme::ModernExecutive,
    };

    let title = args.get("title").and_then(|v| v.as_str()).map(String::from);
    let author = args
        .get("author")
        .and_then(|v| v.as_str())
        .map(String::from);
    let path = PathBuf::from(output_path_str);

    let res = convert_document(input_content, &path, None, theme, title, author)?;

    Ok(format!(
        "Successfully generated {:?} document at '{}' ({} bytes, rendered in {}ms)",
        res.format, res.output_path, res.bytes_written, res.duration_ms
    ))
}

fn handle_list_themes_tool() -> Result<String> {
    Ok(json!({
        "themes": [
            { "id": "modern-executive", "name": "Modern Executive", "description": "Sleek blue accent with clean sans-serif layout" },
            { "id": "tech-spec", "name": "Technical Spec", "description": "Monospace-focused layout ideal for API docs & RFCs" },
            { "id": "minimal-paper", "name": "Minimal Paper", "description": "Elegant serif typography for articles and essays" },
            { "id": "corporate-slate", "name": "Corporate Slate", "description": "Navy header tables and corporate document styling" },
            { "id": "dark-glass", "name": "Dark Glassmorphism", "description": "Vibrant dark background with glowing syntax highlighting" }
        ]
    }).to_string())
}
