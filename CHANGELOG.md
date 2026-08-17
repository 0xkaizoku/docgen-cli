# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-08-17

### 🚀 Added
- **Core Document Engines**:
  - Sub-millisecond **Markdown $\rightarrow$ HTML** renderer with Syntect syntax highlighting.
  - Sub-millisecond **Markdown $\rightarrow$ DOCX (Word)** generator with headings, tables, bullet/ordered lists, blockquotes, and code blocks.
  - Sub-millisecond **JSON/CSV/Markdown $\rightarrow$ XLSX (Excel)** generator with automatic column-width calculation and zebra striping.
  - Resilient **Markdown $\rightarrow$ PDF** generator with macOS native print, Headless Chrome/Edge, WeasyPrint, and wkhtmltopdf pipelines.
- **5 Built-in Theme Presets**:
  - `modern-executive`: Indigo & slate executive report layout.
  - `tech-spec`: Monospace-focused layout for RFCs and technical documentation.
  - `minimal-paper`: Serif typography for academic articles and formal essays.
  - `corporate-slate`: Classic navy headers for business and financial tables.
  - `dark-glass`: Glassmorphism dark mode with glowing accents.
- **Multi-Interface Architecture**:
  - **Standalone CLI**: Works in Unix pipes (`-`), scripts, and terminal directly (`docgen-cli`, `docgen`, `doc`).
  - **Rust Library (`[lib]`)**: Public programmatic crate API (`docgen_cli`) for web servers and Rust tools.
  - **Model Context Protocol (MCP)**: Zero-friction stdio server (`docgen-cli mcp`) for AI tools (Claude Code, Grok, Cursor, Codex).
- **Tooling**:
  - POSIX-compliant multi-arch `install.sh`.
  - Full suite of unit, integration, and doc tests.
  - Complete documentation guides in `docs/`.
