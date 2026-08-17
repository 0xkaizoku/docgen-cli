use clap::Parser;
use docgen_cli::cli::{Cli, Commands, OutputFormat, Theme};
use std::path::Path;

#[test]
fn test_cli_parsing_convert_subcommand() {
    let args = vec![
        "docgen",
        "convert",
        "input.md",
        "-o",
        "output.docx",
        "--theme",
        "tech-spec",
        "--title",
        "Custom Title",
        "--author",
        "Jane Doe",
    ];

    let cli = Cli::try_parse_from(args).expect("Should parse convert command");

    match cli.command {
        Some(Commands::Convert {
            input,
            text,
            output,
            theme,
            format,
            title,
            author,
        }) => {
            assert_eq!(input.unwrap().to_str().unwrap(), "input.md");
            assert_eq!(text, None);
            assert_eq!(output.to_str().unwrap(), "output.docx");
            assert_eq!(theme, Theme::TechSpec);
            assert_eq!(format, None);
            assert_eq!(title, Some("Custom Title".to_string()));
            assert_eq!(author, Some("Jane Doe".to_string()));
        }
        _ => panic!("Expected Commands::Convert"),
    }
}

#[test]
fn test_cli_parsing_inline_text() {
    let args = vec![
        "docgen",
        "convert",
        "--text",
        "# Quick Note\nDirect string",
        "-o",
        "quick.docx",
    ];

    let cli = Cli::try_parse_from(args).expect("Should parse inline text");
    match cli.command {
        Some(Commands::Convert { text, output, .. }) => {
            assert_eq!(text, Some("# Quick Note\nDirect string".to_string()));
            assert_eq!(output.to_str().unwrap(), "quick.docx");
        }
        _ => panic!("Expected Commands::Convert"),
    }
}

#[test]
fn test_cli_parsing_direct_flags() {
    let args = vec![
        "docgen",
        "-i",
        "report.md",
        "-o",
        "report.pdf",
        "--theme",
        "dark-glass",
        "--json",
        "-q",
    ];

    let cli = Cli::try_parse_from(args).expect("Should parse direct flags");
    assert_eq!(cli.input.unwrap().to_str().unwrap(), "report.md");
    assert_eq!(cli.output.unwrap().to_str().unwrap(), "report.pdf");
    assert_eq!(cli.theme, Theme::DarkGlass);
    assert!(cli.json);
    assert!(cli.quiet);
}

#[test]
fn test_cli_parsing_mcp_and_init() {
    let mcp_cli = Cli::try_parse_from(vec!["docgen", "mcp"]).unwrap();
    match mcp_cli.command {
        Some(Commands::Mcp) => {}
        _ => panic!("Expected Commands::Mcp"),
    }

    let init_cli = Cli::try_parse_from(vec!["docgen", "init-ai"]).unwrap();
    match init_cli.command {
        Some(Commands::InitAi { global }) => {
            assert!(global);
        }
        _ => panic!("Expected Commands::InitAi"),
    }

    let tmpl_cli = Cli::try_parse_from(vec!["docgen", "list-templates"]).unwrap();
    match tmpl_cli.command {
        Some(Commands::ListTemplates) => {}
        _ => panic!("Expected Commands::ListTemplates"),
    }
}

#[test]
fn test_format_inference_from_paths() {
    assert_eq!(
        OutputFormat::from_path(Path::new("file.pdf")),
        Some(OutputFormat::Pdf)
    );
    assert_eq!(
        OutputFormat::from_path(Path::new("file.PDF")),
        Some(OutputFormat::Pdf)
    );
    assert_eq!(
        OutputFormat::from_path(Path::new("dir/report.docx")),
        Some(OutputFormat::Docx)
    );
    assert_eq!(
        OutputFormat::from_path(Path::new("data.xlsx")),
        Some(OutputFormat::Xlsx)
    );
    assert_eq!(
        OutputFormat::from_path(Path::new("data.xls")),
        Some(OutputFormat::Xlsx)
    );
    assert_eq!(
        OutputFormat::from_path(Path::new("preview.html")),
        Some(OutputFormat::Html)
    );
    assert_eq!(
        OutputFormat::from_path(Path::new("preview.htm")),
        Some(OutputFormat::Html)
    );
    assert_eq!(OutputFormat::from_path(Path::new("file.txt")), None);
    assert_eq!(OutputFormat::from_path(Path::new("no_extension")), None);
}

#[test]
fn test_theme_properties() {
    let themes = [
        Theme::ModernExecutive,
        Theme::TechSpec,
        Theme::MinimalPaper,
        Theme::CorporateSlate,
        Theme::DarkGlass,
    ];

    for theme in themes {
        assert!(!theme.name().is_empty());
        assert!(!theme.description().is_empty());
    }
}
