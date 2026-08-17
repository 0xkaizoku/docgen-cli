# 📖 `docgen` CLI User Guide

`docgen` is a universal, zero-dependency document generation CLI. It runs locally from your terminal, with piped command outputs, in shell scripts, and in build workflows, converting Markdown, JSON, CSV, and tabular data into styled **DOCX**, **XLSX**, **HTML**, and **PDF** documents in milliseconds.

---

## 🚀 Basic Syntax

```bash
# Convert a file with automatic format inference based on output extension
docgen convert <INPUT> -o <OUTPUT>

# Shorthand binary alias
doc convert <INPUT> -o <OUTPUT>

# Direct flag syntax
docgen -i <INPUT> -o <OUTPUT>
```

---

## 📄 Converting Documents

### 1. Markdown to Word Document (`.docx`)
Generates Microsoft Word documents with structured headings, formatted tables, bullet/numbered lists, callouts, and code blocks:

```bash
# Convert markdown report to DOCX
docgen convert report.md -o report.docx

# Set title, author, and theme
docgen convert spec.md -o spec.docx --theme tech-spec --title "System Architecture" --author "Platform Team"
```

### 2. JSON & Tables to Excel Spreadsheet (`.xlsx`)
Converts structured JSON (arrays of objects or arrays of arrays), CSV files, TSV files, or Markdown tables directly into styled Excel spreadsheets with automated column widths and zebra striping:

```bash
# Convert JSON array of objects to Excel
docgen convert sales_data.json -o sales_q3.xlsx

# Convert CSV or TSV data
docgen convert metrics.csv -o metrics.xlsx

# Convert Markdown table
docgen convert table.md -o summary.xlsx
```

### 3. Markdown to Styled HTML (`.html`)
Renders standalone HTML with Google Fonts typography, syntax-highlighted code blocks, responsive layouts, and print styles:

```bash
# Convert markdown to HTML preview
docgen convert article.md -o article.html --theme minimal-paper

# Dark glassmorphism mode
docgen convert readme.md -o preview.html --theme dark-glass
```

### 4. Markdown to PDF (`.pdf`)
Renders vector-quality PDF documents using native OS print pipelines or headless browser rendering:

```bash
# Convert report to PDF
docgen convert report.md -o report.pdf --theme modern-executive
```

### 5. Convert Inline Text Directly (`--text`)
Generate documents from raw strings or terminal variables without creating any file:

```bash
# Convert inline markdown text
docgen convert --text "# Server Outage Report\nIncident resolved at 22:45." -o report.docx

# Convert inline JSON array into Excel
docgen convert --text '[{"Project": "Apollo", "Budget": 120000}, {"Project": "Hermes", "Budget": 95000}]' -o budget.xlsx
```

---

## ⚡ Converting CLI Results & Unix Command Pipelines

`docgen` can take input from standard input (`-`) or implicit pipes, allowing you to turn **any terminal command's output** directly into a styled document:

```bash
# 1. Pipe Git Commit Logs to a Word Document (.docx)
git log -n 30 --pretty=format:"%h - %an (%ar): %s" | docgen convert - -o changelog.docx --title "Recent Commits"

# 2. Pipe GitHub API JSON directly into an Excel Spreadsheet (.xlsx)
curl -s https://api.github.com/repos/0xkaizoku/docgen-cli/commits | docgen convert - -o commits.xlsx

# 3. Pipe System Process / Resource Tables to Excel (.xlsx)
ps aux | head -n 40 | docgen convert - -o running_processes.xlsx

# 4. Pipe Docker Container Status to HTML (.html)
docker ps --format "table {{.ID}}\t{{.Image}}\t{{.Status}}\t{{.Ports}}" | docgen convert - -o docker_status.html

# 5. Pipe Network or Disk Space Stats to PDF (.pdf)
df -h | docgen convert - -o disk_usage.pdf --theme tech-spec

# 6. Stream HTML Output to Stdout for Browser Preview / Other Tools
docgen convert report.md -o - --format html > output.html
```

---

## 🎨 Themes & Design Presets

List all available built-in themes:
```bash
docgen list-templates
```

| Theme | Flag | Best For |
| :--- | :--- | :--- |
| **Modern Executive** *(Default)* | `--theme modern-executive` | Executive summaries, business proposals, reports |
| **Technical Spec** | `--theme tech-spec` | RFCs, API documentation, developer specs |
| **Minimal Paper** | `--theme minimal-paper` | Academic papers, essays, articles |
| **Corporate Slate** | `--theme corporate-slate` | Financial spreadsheets, legal documents, tables |
| **Dark Glass** | `--theme dark-glass` | Modern developer previews, dark mode documents |

---

## 🛠️ CLI Options Reference

```text
Usage: docgen [COMMAND] [OPTIONS]

Commands:
  convert         Convert document from input file, stdin pipe, or inline text
  mcp             Run Model Context Protocol (MCP) server over Stdio
  init-ai         Auto-configure local AI CLI environments (Claude, Grok, Cursor)
  list-templates  List all available built-in design themes
  help            Print help information

Options:
  -i, --input <FILE>    Input file path (use '-' for stdin)
      --text <STRING>   Direct inline text / string input
  -o, --output <FILE>   Output file path (use '-' for stdout)
  -t, --theme <THEME>   Theme preset [default: modern-executive]
  -f, --format <FORMAT> Explicit target format [pdf, docx, xlsx, html]
      --json            Output machine-readable execution stats in JSON
  -q, --quiet           Quiet mode (suppresses status logs)
  -h, --help            Print help
  -V, --version         Print version
```

---

## 🤖 Machine-Readable Mode (`--json`)

For integration into automated scripts or build pipelines, pass `--json`:

```bash
docgen convert input.md -o output.docx --json
```

**Output:**
```json
{
  "bytes_written": 26412,
  "duration_ms": 1,
  "format": "docx",
  "output_path": "output.docx",
  "status": "success"
}
```
