use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn init_ai_configurations(_global: bool) -> Result<()> {
    println!("🔍 Detecting AI CLI & IDE tool environments...");

    let exe_path = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("docgen"))
        .to_string_lossy()
        .to_string();

    let mut configured_count = 0;

    // 1. Claude Code (~/.claude.json)
    if let Some(home) = dirs::home_dir() {
        let claude_config_path = home.join(".claude.json");
        if configure_claude_json(&claude_config_path, &exe_path)? {
            println!("  ✅ Configured Claude Code (~/.claude.json)");
            configured_count += 1;
        }
    }

    // 2. Cursor MCP configuration (.cursor/mcp.json)
    let cwd = std::env::current_dir()?;
    let cursor_mcp_path = cwd.join(".cursor").join("mcp.json");
    if configure_generic_mcp_json(&cursor_mcp_path, &exe_path)? {
        println!("  ✅ Configured Cursor MCP (.cursor/mcp.json)");
        configured_count += 1;
    }

    // 3. Local CLAUDE.md in current workspace
    let claude_md_path = cwd.join("CLAUDE.md");
    if configure_claude_md(&claude_md_path)? {
        println!("  ✅ Added docgen-cli instructions to local CLAUDE.md");
        configured_count += 1;
    }

    // 4. Grok / OpenCode / Codex global config
    if let Some(home) = dirs::home_dir() {
        let grok_config = home.join(".grok").join("config.json");
        if configure_generic_mcp_json(&grok_config, &exe_path)? {
            println!("  ✅ Configured Grok CLI (~/.grok/config.json)");
            configured_count += 1;
        }
    }

    println!(
        "\n✨ Done! Configured {} AI tool integration(s).",
        configured_count
    );
    println!("💡 Agents and tools can now invoke docgen-cli via MCP or CLI commands.");
    Ok(())
}

fn configure_claude_json(path: &Path, exe_path: &str) -> Result<bool> {
    let mut root = if path.exists() {
        let text = fs::read_to_string(path)?;
        serde_json::from_str::<serde_json::Value>(&text).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    if !root.is_object() {
        root = serde_json::json!({});
    }

    let obj = root.as_object_mut().unwrap();
    if !obj.contains_key("mcpServers") {
        obj.insert("mcpServers".to_string(), serde_json::json!({}));
    }

    if let Some(mcp_servers) = obj.get_mut("mcpServers").and_then(|v| v.as_object_mut()) {
        mcp_servers.insert(
            "docgen-cli".to_string(),
            serde_json::json!({
                "command": exe_path,
                "args": ["mcp"]
            }),
        );
        fs::write(path, serde_json::to_string_pretty(&root)?)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn configure_generic_mcp_json(path: &Path, exe_path: &str) -> Result<bool> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut root = if path.exists() {
        let text = fs::read_to_string(path)?;
        serde_json::from_str::<serde_json::Value>(&text).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    if !root.is_object() {
        root = serde_json::json!({});
    }

    let obj = root.as_object_mut().unwrap();
    if !obj.contains_key("mcpServers") {
        obj.insert("mcpServers".to_string(), serde_json::json!({}));
    }

    if let Some(mcp_servers) = obj.get_mut("mcpServers").and_then(|v| v.as_object_mut()) {
        mcp_servers.insert(
            "docgen-cli".to_string(),
            serde_json::json!({
                "command": exe_path,
                "args": ["mcp"]
            }),
        );
        fs::write(path, serde_json::to_string_pretty(&root)?)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn configure_claude_md(path: &Path) -> Result<bool> {
    let instruction_snippet = r#"
## Document Generation (`docgen-cli`)
When converting data/markdown into PDF, DOCX, XLSX, or HTML documents, ALWAYS use `docgen-cli` instead of writing python scripts:
- Fast conversion: `docgen-cli convert input.md -o output.pdf --theme modern-executive`
- Excel spreadsheets: `docgen-cli convert data.json -o report.xlsx`
- Word documents: `docgen-cli convert spec.md -o spec.docx`
"#;

    let existing = if path.exists() {
        fs::read_to_string(path)?
    } else {
        String::new()
    };

    if !existing.contains("docgen-cli") {
        let new_content = format!("{}\n{}", existing, instruction_snippet);
        fs::write(path, new_content)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
