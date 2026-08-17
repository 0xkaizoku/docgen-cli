use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "docgen",
    author = "Kaizoku",
    version = "0.1.0",
    about = "Blazing-fast zero-dependency universal document engine & MCP server",
    long_about = "docgen converts Markdown, JSON, CSV, TSV, and terminal command outputs into styled PDF, DOCX, XLSX, and HTML documents in milliseconds.\n\nWorks standalone from your terminal, with piped CLI results, in shell scripts, as a Rust library, or as an MCP server for AI tools."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Input file path (use '-' for stdin; optional if --text is provided)
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Direct inline text / string input (bypasses file reading)
    #[arg(long, value_name = "STRING")]
    pub text: Option<String>,

    /// Output file path (e.g. output.pdf, report.docx, data.xlsx; use '-' for stdout)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Document theme / styling preset
    #[arg(short, long, value_enum, default_value_t = Theme::ModernExecutive)]
    pub theme: Theme,

    /// Explicit target format (auto-inferred from output file extension if omitted)
    #[arg(short, long, value_enum)]
    pub format: Option<OutputFormat>,

    /// Title of document
    #[arg(long)]
    pub title: Option<String>,

    /// Author name
    #[arg(long)]
    pub author: Option<String>,

    /// Output execution stats in JSON format
    #[arg(long)]
    pub json: bool,

    /// Quiet mode (suppress informative output)
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert document from file, stdin pipe, or inline text to target format
    Convert {
        /// Input file path (or '-' for stdin; optional if --text is provided)
        #[arg(value_name = "INPUT")]
        input: Option<PathBuf>,

        /// Direct inline text, markdown, JSON, or CSV string
        #[arg(long, value_name = "STRING")]
        text: Option<String>,

        /// Output file path (or '-' for stdout)
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,

        /// Document theme / styling preset
        #[arg(short, long, value_enum, default_value_t = Theme::ModernExecutive)]
        theme: Theme,

        /// Explicit target format (pdf, docx, xlsx, html)
        #[arg(short, long, value_enum)]
        format: Option<OutputFormat>,

        /// Title of document
        #[arg(long)]
        title: Option<String>,

        /// Author name
        #[arg(long)]
        author: Option<String>,
    },

    /// Run Model Context Protocol (MCP) server over Stdio for AI tools
    Mcp,

    /// Auto-detect and configure local AI CLI tools (Claude Code, Grok, Cursor, etc.)
    InitAi {
        /// Install tools globally in user home directory
        #[arg(long, default_value_t = true)]
        global: bool,
    },

    /// List all available built-in design themes and templates
    ListTemplates,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Pdf,
    Docx,
    Xlsx,
    Html,
}

impl OutputFormat {
    pub fn from_path(path: &std::path::Path) -> Option<Self> {
        match path.extension()?.to_str()?.to_lowercase().as_str() {
            "pdf" => Some(OutputFormat::Pdf),
            "docx" => Some(OutputFormat::Docx),
            "xlsx" | "xls" => Some(OutputFormat::Xlsx),
            "html" | "htm" => Some(OutputFormat::Html),
            _ => None,
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    ModernExecutive,
    TechSpec,
    MinimalPaper,
    CorporateSlate,
    DarkGlass,
}

impl Theme {
    pub fn name(&self) -> &'static str {
        match self {
            Theme::ModernExecutive => "Modern Executive",
            Theme::TechSpec => "Technical Spec",
            Theme::MinimalPaper => "Minimalist Typography",
            Theme::CorporateSlate => "Corporate Slate",
            Theme::DarkGlass => "Dark Glassmorphism",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Theme::ModernExecutive => "Sleek indigo/blue theme for executive reports and summaries",
            Theme::TechSpec => {
                "Monospace typography optimized for RFCs and technical documentation"
            }
            Theme::MinimalPaper => {
                "Serif typography for academic papers, articles, and formal essays"
            }
            Theme::CorporateSlate => "Classic navy headers for business, legal, and financial docs",
            Theme::DarkGlass => {
                "Glassmorphism dark mode with glowing accents and syntax highlights"
            }
        }
    }
}
