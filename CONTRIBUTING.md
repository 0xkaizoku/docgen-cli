# Contributing to `docgen-cli`

Thank you for your interest in contributing to `docgen-cli`! We welcome contributions of all kinds—bug fixes, new document formatting features, performance enhancements, documentation improvements, and theme additions.

---

## 🛠️ Development Setup

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (Stable 1.75+ recommended)
- `git`

### Quickstart
1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/0xkaizoku/docgen-cli.git
   cd docgen-cli
   ```
2. **Build the codebase:**
   ```bash
   cargo build
   ```
3. **Run the test suite:**
   ```bash
   cargo test --all-targets
   ```

---

## 🧪 Code Quality & Testing

Before submitting a pull request, ensure all checks pass:

```bash
# 1. Format code according to Rustfmt standards
cargo fmt --check

# 2. Run Clippy to detect common mistakes and idioms
cargo clippy --all-targets -- -D warnings

# 3. Run all unit, integration, and doc tests
cargo test --all-targets
```

---

## 📁 Codebase Structure

- `src/lib.rs`: Public Rust crate exports (`docgen_cli`) and core conversion coordinator.
- `src/main.rs`: CLI binary entry point (`docgen-cli`).
- `src/bin/docgen.rs`: Binary alias (`docgen`).
- `src/bin/doc.rs`: Shorthand binary alias (`doc`).
- `src/cli.rs`: Command line arguments, Clap definitions, and theme enums.
- `src/engine/`: Core conversion engines:
  - `docx.rs`: Microsoft Word OOXML document generation.
  - `xlsx.rs`: Microsoft Excel spreadsheet generation with auto-column width.
  - `html.rs`: Responsive HTML renderer with Syntect code syntax highlighting.
  - `pdf.rs`: Multi-pipeline PDF generator (macOS native, headless Chrome/Edge, WeasyPrint, wkhtmltopdf).
- `src/templates/`: Design presets, CSS stylesheets, and HTML wrappers.
- `src/mcp.rs`: Model Context Protocol JSON-RPC Stdio server.
- `src/installer.rs`: AI CLI auto-configuration helper.
- `tests/`: Integration test suites for CLI and engines.
- `docs/`: In-depth developer, CLI, MCP, and theme guides.

---

## 📄 License
By contributing to `docgen-cli`, you agree that your contributions will be licensed under the [Docgen-CLI Protective Free-Use License](LICENSE).
