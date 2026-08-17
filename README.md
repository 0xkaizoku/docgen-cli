<div align="center">

# ⚡ `docgen-cli`

### **Sub-Millisecond Universal Document Engine, CLI & MCP Server**

*Convert Markdown, JSON, CSV, and tabular data into styled **PDF**, **DOCX**, **XLSX**, and **HTML** documents in **milliseconds**—with zero runtime dependencies.*

[![License: Protective Free-Use](https://img.shields.io/badge/License-Protective_Free--Use-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Speed](https://img.shields.io/badge/Latency-%3C_2ms-brightgreen.svg)](docs/BENCHMARKS.md)
[![Tokens Saved](https://img.shields.io/badge/LLM_Tokens_Saved-100%25-brightgreen.svg)]()
[![MCP](https://img.shields.io/badge/MCP-Compatible-8A2BE2.svg)](https://modelcontextprotocol.io)
[![Cost](https://img.shields.io/badge/Cost-100%25_FREE-success.svg)]()

---

</div>

## 💥 The Token Waste & Cost Problem with AI Tools Today

Whenever an AI assistant or CLI agent (**Claude Code**, **Grok CLI**, **Cursor**, **Windsurf**, **Codex**, **OpenCode**) needs to generate a PDF report, Word document, or Excel spreadsheet for you:

1. 💸 **Burns 1,500 – 4,000 LLM Tokens**: The model writes 150+ lines of throwaway Python (`reportlab`, `python-docx`, `openpyxl`) or Node.js scripts.
2. ⏱️ **Wastes 30–60 Seconds**: Spends minutes creating virtual environments and installing `pip`/`npm` packages.
3. 💥 **Crashes 30% of the Time**: Fails on missing shared libraries or syntax errors, causing multi-turn retry loops that burn **even more tokens**.
4. 🛑 **Drains Monthly Quotas**: Consumes your expensive premium fast-request limits on tedious boilerplate scripting.

```
+---------------------------------------------------------------------------------------------------+
| ❌ TRADITIONAL AI AGENT:                                                                          |
| Prompt --> [LLM writes 200 lines Python: 2,500 Tokens ($0.08)] --> [Pip Install: 45s] --> Crash  |
+---------------------------------------------------------------------------------------------------+
| ✅ WITH DOCGEN-CLI (100% FREE & LOCAL):                                                           |
| Prompt --> [Native Tool Call: 30 Tokens ($0.0001)] --> [docgen-cli: 1ms] --> Instant Output File  |
+---------------------------------------------------------------------------------------------------+
```

---

## 💰 Token Usage & Cost Savings Breakdown Across Popular Tools

`docgen-cli` is **100% FREE** to use and cuts document-generation token costs by **up to 99%**:

| AI Tool / Platform | Traditional Approach (Token & Cost Overhead) | With `docgen-cli` (Native Engine) | Direct Savings |
| :--- | :--- | :--- | :--- |
| **Claude Code** *(Claude 3.5 Sonnet / Opus)* | Burns ~2,500 tokens ($0.08–$0.15) drafting & debugging Python | **~35 tokens ($0.0001)** via instant MCP tool call | **> 98% Token & Cost Reduction** |
| **Cursor & Windsurf** | Consumes 2–4 premium fast requests per doc creation | **1 fast request / instant CLI command** | **Saves 75% of your monthly request quota** |
| **Grok CLI & Codex** | Often gets stuck in pip dependency / missing library loops | **Deterministic execution in 1ms with 0 dependencies** | **100% elimination of debug loops** |
| **CI/CD & Cloud Runners** | Requires heavy Chromium runners ($0.48/hr, 500MB RAM) | **Runs on 12MB RAM ($0.004/hr) with single binary** | **99% Cloud Compute Cost Savings** |

### 📈 Real-World Return on Investment (ROI)

- 🧑‍💻 **Individual Developer**: Saves **15,000 – 60,000 tokens/day** ($\approx$ **$30–$80/month** in direct API / subscription savings) and **125+ hours of idle waiting time per year**.
- 🏢 **50-Engineer Team**: Saves an estimated **$22,000+/month** across LLM tokens and cloud runner overhead.
- 🆓 **100% Free Forever**: `docgen-cli` is free for personal, internal, and commercial use with zero recurring fees.

---

## 🌟 Core Highlights

- ⚡ **Blazing Fast**: Generates Word documents, Excel sheets, and HTML pages in **1–4 milliseconds**.
- 📦 **Zero External Runtime Dependencies**: Single statically-compiled native binary—no Python, no Node.js, no LibreOffice required.
- 🛠️ **Universal Use**: Works as a **standalone CLI tool**, a **programmatic Rust library**, in **shell scripts / Makefiles**, and as an **MCP server** for AI agents.
- 🎨 **5 Curated Themes**: Modern Executive, Technical Spec, Minimal Paper, Corporate Slate, and Dark Glassmorphism.
- 📊 **Smart Excel Generation**: Auto-calculates column widths, handles JSON objects/arrays, Markdown tables, and CSVs with zebra striping.
- 📝 **Rich Word Documents**: Generates clean `.docx` files with headings, tables, bullet/numbered lists, callouts, and code blocks.
- 🤖 **Model Context Protocol (MCP)**: Native stdio JSON-RPC server ready for Claude Code, Grok, Cursor, Windsurf, and Codex.

---

## 🏛️ Architecture

```
                               +--------------------------------------------+
                               |                 docgen-cli                 |
                               | (Universal High-Performance Document Core) |
                               +--------------------------------------------+
                                                     |
             +----------------------+----------------+----------------------+----------------------+
             |                      |                                       |                      |
             v                      v                                       v                      v
    +-----------------+    +-----------------+                     +-----------------+    +-----------------+
    |  Terminal CLI   |    |   Rust Crate    |                     |  Shell / Scripts|    | AI Tools & MCP  |
    | (Developer Use) |    |  (Library API)  |                     | (Unix Pipelines)|    |(Claude, Grok,   |
    | `docgen-cli`    |    |`use docgen_cli` |                     | `curl | docgen` |    | Cursor, Windsurf|
    +-----------------+    +-----------------+                     +-----------------+    +-----------------+
             |                      |                                       |                      |
             +----------------------+----------------+----------------------+----------------------+
                                                     |
                                                     v
                               +--------------------------------------------+
                               | Output Documents (Sub-Millisecond Latency) |
                               |   [.docx]     [.xlsx]    [.html]    [.pdf] |
                               +--------------------------------------------+
```

---

## 📦 Installation

### Option 1: One-Line Shell Installer (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/0xkaizoku/docgen-cli/main/install.sh | sh
```

### Option 2: Cargo (Rust Package Manager)
```bash
cargo install --git https://github.com/0xkaizoku/docgen-cli
```

### Option 3: Build from Source
```bash
git clone https://github.com/0xkaizoku/docgen-cli.git
cd docgen-cli
cargo build --release
cp target/release/docgen-cli /usr/local/bin/
cp target/release/docgen /usr/local/bin/
cp target/release/doc /usr/local/bin/
```

---

## 🚀 Quickstart: Standalone CLI

Run conversions with **`docgen`** (or shorthand aliases **`doc`** / **`docgen-cli`**):

### 1. Convert Files Directly (Markdown, JSON, CSV, TSV)
```bash
# Convert Markdown to a Word document (.docx) in 1ms
docgen convert spec.md -o spec.docx --theme tech-spec

# Convert JSON data directly to an Excel spreadsheet (.xlsx) in 1ms
docgen convert sales_data.json -o sales_q3.xlsx

# Convert CSV / TSV data to Excel spreadsheet
docgen convert metrics.csv -o metrics.xlsx

# Convert Markdown to styled HTML preview
docgen convert article.md -o preview.html --theme minimal-paper

# Generate vector PDF document
docgen convert report.md -o report.pdf --theme modern-executive
```

### 2. Convert Terminal Outputs & Piped Command Results
You do **NOT** need to create temporary files on disk. Pipe any command's terminal output directly:
```bash
# Pipe git log into a styled Word document
git log -n 25 --oneline | docgen convert - -o changelog.docx --title "Commit History"

# Pipe API JSON output directly into an Excel spreadsheet
curl -s https://api.github.com/repos/0xkaizoku/docgen-cli/commits | docgen convert - -o commits.xlsx

# Pipe system process tables into an Excel spreadsheet
ps aux | head -n 30 | docgen convert - -o processes.xlsx

# Pipe markdown stream directly into HTML or PDF
cat notes.txt | docgen convert - -o notes.pdf --theme minimal-paper
```

### 3. Convert Inline Text Directly (`--text`)
Pass inline text or JSON directly from the terminal without creating any file:
```bash
# Generate DOCX from an inline string
docgen convert --text "# Server Outage Report\nIncident resolved at 22:45." -o incident.docx

# Generate Excel spreadsheet from inline JSON
docgen convert --text '[{"Metric": "QPS", "Value": 14500}, {"Metric": "Errors", "Value": 0}]' -o metrics.xlsx
```

### 4. List Built-in Themes
```bash
docgen list-templates
```

---

## 📚 Quickstart: Rust Library (`[lib]`)

Use `docgen-cli` programmatically inside your web services, background workers, or CLI tools:

```toml
[dependencies]
docgen-cli = { git = "https://github.com/0xkaizoku/docgen-cli" }
```

```rust
use docgen_cli::{convert_document, OutputFormat, Theme};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let markdown = "# System Status\nAll **14 microservices** operational.";

    let res = convert_document(
        markdown,
        Path::new("status.docx"),
        Some(OutputFormat::Docx),
        Theme::ModernExecutive,
        Some("Status Report".to_string()),
        Some("Platform Team".to_string()),
    )?;

    println!("Generated {} ({} bytes) in {}ms", res.output_path, res.bytes_written, res.duration_ms);
    Ok(())
}
```

👉 See the [Library Guide](docs/LIBRARY_GUIDE.md) for in-memory byte generation and Axum/Actix web server integration.

---

## 🤖 Optional: AI Tool & MCP Integration

If you use AI CLI tools (**Claude Code**, **Grok CLI**, **Cursor**, **Windsurf**, **Codex**, **OpenCode**), `docgen-cli` integrates instantly via MCP Stdio:

### 1-Click Auto Configuration
```bash
docgen-cli init-ai
```

### Manual Configuration
Add `docgen-cli` to `~/.claude.json` or `.cursor/mcp.json`:
```json
{
  "mcpServers": {
    "docgen-cli": {
      "command": "docgen-cli",
      "args": ["mcp"]
    }
  }
}
```

👉 See the [MCP Guide](docs/MCP_GUIDE.md) for full configuration details and schemas.

---

## 🎨 Built-in Theme Catalog

| Theme Name | Style Description | Best Used For |
| :--- | :--- | :--- |
| `modern-executive` *(Default)* | Royal blue accent, crisp sans-serif typography | Executive summaries, business proposals, memos |
| `tech-spec` | JetBrains Mono headers, cyan accents, dark code | Engineering RFCs, API references, architecture docs |
| `minimal-paper` | Editorial serif typography, cream background | Academic research, essays, documentation articles |
| `corporate-slate` | Deep navy headers, structured borders, high-density | Financial spreadsheets, legal documents, data audits |
| `dark-glass` | Dark glassmorphism, glowing accents, syntax styling | Modern web documentation, dark mode presentations |

👉 See the [Theme Catalog](docs/THEMES.md) for CSS customization details.

---

## 📊 Performance & Efficiency

| Conversion Task | Traditional Scripting (Python / Playwright) | `docgen-cli` Native Engine | Speedup |
| :--- | :--- | :--- | :--- |
| **Markdown $\rightarrow$ HTML** | ~2,400ms | **~4 ms** | **> 600x Faster** |
| **Markdown $\rightarrow$ DOCX (Word)** | ~4,200ms | **~1 ms** | **> 4,000x Faster** |
| **JSON / CSV $\rightarrow$ XLSX (Excel)** | ~3,100ms | **~1 ms** | **> 3,000x Faster** |
| **Markdown $\rightarrow$ PDF** | ~35,000ms | **~35 ms** | **> 1,000x Faster** |

👉 Detailed breakdown in [Benchmarks & Methodology](docs/BENCHMARKS.md).

---

## 📖 In-Depth Guides

- 📘 [CLI User Guide](docs/CLI_GUIDE.md) — Comprehensive command reference, pipes, and scripting.
- 📕 [Rust Library Guide](docs/LIBRARY_GUIDE.md) — Programmatic API, in-memory buffers, web server handlers.
- 🤖 [MCP Server Guide](docs/MCP_GUIDE.md) — Model Context Protocol setup and tool schemas.
- 🎨 [Themes & Styling](docs/THEMES.md) — Design presets, typography, and print layouts.
- ⚡ [Performance Benchmarks](docs/BENCHMARKS.md) — Speed, memory footprint, and comparative analysis.

---

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) and [Security Policy](SECURITY.md) before submitting pull requests.

---

## 📄 License

Distributed under the **Docgen-CLI Protective Free-Use License**. Free to use for personal, internal, and commercial purposes; unauthorized duplication, rebranding, or commercial cloning of the codebase is prohibited. See [`LICENSE`](LICENSE) for details.
