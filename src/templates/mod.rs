use crate::cli::Theme;

#[derive(Debug, Clone)]
pub struct DocumentMeta {
    pub title: String,
    pub author: Option<String>,
    pub date: Option<String>,
    pub theme: Theme,
}

impl Default for DocumentMeta {
    fn default() -> Self {
        Self {
            title: "Document".to_string(),
            author: None,
            date: None,
            theme: Theme::ModernExecutive,
        }
    }
}

pub fn get_theme_css(theme: Theme) -> &'static str {
    match theme {
        Theme::ModernExecutive => MODERN_EXECUTIVE_CSS,
        Theme::TechSpec => TECH_SPEC_CSS,
        Theme::MinimalPaper => MINIMAL_PAPER_CSS,
        Theme::CorporateSlate => CORPORATE_SLATE_CSS,
        Theme::DarkGlass => DARK_GLASS_CSS,
    }
}

pub fn wrap_html(content_html: &str, meta: &DocumentMeta) -> String {
    let css = get_theme_css(meta.theme);
    let author_meta = meta
        .author
        .as_ref()
        .map(|a| format!("<meta name=\"author\" content=\"{}\">", html_escape(a)))
        .unwrap_or_default();

    let date_str = meta.date.clone().unwrap_or_else(current_date_string);

    let header_subtitle = if let Some(ref author) = meta.author {
        format!(
            "<div class=\"doc-byline\"><span>By <strong>{}</strong></span> &bull; <span>{}</span></div>",
            html_escape(author),
            html_escape(&date_str)
        )
    } else {
        format!(
            "<div class=\"doc-byline\"><span>{}</span></div>",
            html_escape(&date_str)
        )
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    {}
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:ital,wght@0,400;0,600;1,400&family=Newsreader:ital,opsz,wght@0,6..72,400;0,6..72,600;1,6..72,400&display=swap" rel="stylesheet">
    <style>
    {}
    </style>
</head>
<body>
    <div class="page-container">
        <header class="doc-header">
            <h1 class="doc-title">{}</h1>
            {}
        </header>
        <main class="doc-body">
            {}
        </main>
        <footer class="doc-footer">
            <span>Generated with <strong>docgen-cli</strong></span>
            <span class="page-number"></span>
        </footer>
    </div>
</body>
</html>"#,
        html_escape(&meta.title),
        author_meta,
        css,
        html_escape(&meta.title),
        header_subtitle,
        content_html
    )
}

fn current_date_string() -> String {
    "August 2026".to_string()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

const MODERN_EXECUTIVE_CSS: &str = r#"
:root {
    --primary: #0f172a;
    --accent: #2563eb;
    --accent-light: #eff6ff;
    --text: #334155;
    --text-muted: #64748b;
    --bg: #ffffff;
    --card-bg: #f8fafc;
    --border: #e2e8f0;
    --font-main: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    --font-code: 'JetBrains Mono', monospace;
}

@page {
    size: A4;
    margin: 20mm;
}

body {
    font-family: var(--font-main);
    color: var(--text);
    background-color: var(--bg);
    line-height: 1.65;
    font-size: 15px;
    margin: 0;
    padding: 0;
    -webkit-font-smoothing: antialiased;
}

.page-container {
    max-width: 860px;
    margin: 0 auto;
    padding: 48px 32px;
}

.doc-header {
    border-bottom: 2px solid var(--accent);
    padding-bottom: 24px;
    margin-bottom: 36px;
}

.doc-title {
    font-size: 2.3rem;
    font-weight: 700;
    color: var(--primary);
    letter-spacing: -0.025em;
    margin: 0 0 10px 0;
}

.doc-byline {
    font-size: 0.95rem;
    color: var(--text-muted);
    font-weight: 400;
}

.doc-body h1, .doc-body h2, .doc-body h3, .doc-body h4 {
    color: var(--primary);
    font-weight: 600;
    letter-spacing: -0.015em;
    margin-top: 1.8em;
    margin-bottom: 0.6em;
}

.doc-body h1 { font-size: 1.8rem; border-bottom: 2px solid var(--border); padding-bottom: 8px; }
.doc-body h2 { font-size: 1.4rem; border-bottom: 1px solid var(--border); padding-bottom: 6px; }
.doc-body h3 { font-size: 1.15rem; }

.doc-body p { margin-bottom: 1.2em; }
.doc-body a { color: var(--accent); text-decoration: none; }
.doc-body a:hover { text-decoration: underline; }

.doc-body blockquote {
    border-left: 4px solid var(--accent);
    background-color: var(--accent-light);
    margin: 1.5em 0;
    padding: 12px 20px;
    border-radius: 0 8px 8px 0;
    color: var(--primary);
}

.doc-body code {
    font-family: var(--font-code);
    font-size: 0.88em;
    background-color: var(--card-bg);
    border: 1px solid var(--border);
    padding: 2px 6px;
    border-radius: 4px;
}

.doc-body pre {
    background-color: #0f172a;
    color: #f8fafc;
    padding: 18px 22px;
    border-radius: 8px;
    overflow-x: auto;
    font-family: var(--font-code);
    font-size: 0.88em;
    line-height: 1.5;
}

.doc-body pre code {
    background: transparent;
    border: none;
    padding: 0;
    color: inherit;
}

.doc-body table {
    width: 100%;
    border-collapse: collapse;
    margin: 1.8em 0;
    font-size: 0.95em;
}

.doc-body th {
    background-color: var(--card-bg);
    color: var(--primary);
    font-weight: 600;
    text-align: left;
    padding: 12px 16px;
    border-bottom: 2px solid var(--border);
}

.doc-body td {
    padding: 10px 16px;
    border-bottom: 1px solid var(--border);
}

.doc-body tr:nth-child(even) {
    background-color: #f9fafb;
}

.doc-footer {
    margin-top: 60px;
    padding-top: 20px;
    border-top: 1px solid var(--border);
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    color: var(--text-muted);
}
"#;

const TECH_SPEC_CSS: &str = r#"
:root {
    --primary: #09090b;
    --accent: #0284c7;
    --text: #27272a;
    --text-muted: #71717a;
    --bg: #ffffff;
    --card-bg: #f4f4f5;
    --border: #e4e4e7;
    --font-main: 'Inter', sans-serif;
    --font-code: 'JetBrains Mono', monospace;
}
@page { size: A4; margin: 20mm; }
body { font-family: var(--font-main); color: var(--text); padding: 40px; max-width: 860px; margin: 0 auto; line-height: 1.6; }
.doc-header { border-left: 6px solid var(--accent); padding-left: 20px; margin-bottom: 30px; }
.doc-title { font-family: var(--font-code); color: var(--primary); font-size: 2rem; margin: 0 0 8px 0; }
.doc-byline { font-family: var(--font-code); font-size: 0.85rem; color: var(--text-muted); }
pre { background: #18181b; color: #38bdf8; padding: 18px; border-radius: 6px; font-family: var(--font-code); overflow-x: auto; }
code { font-family: var(--font-code); background: var(--card-bg); padding: 2px 5px; border-radius: 4px; font-size: 0.9em; }
table { width: 100%; border-collapse: collapse; margin: 20px 0; }
th, td { border: 1px solid var(--border); padding: 10px 14px; text-align: left; }
th { background: var(--card-bg); font-weight: 600; }
.doc-footer { margin-top: 40px; padding-top: 15px; border-top: 1px solid var(--border); font-size: 0.8rem; color: var(--text-muted); }
"#;

const MINIMAL_PAPER_CSS: &str = r#"
:root {
    --primary: #1c1917;
    --text: #292524;
    --text-muted: #78716c;
    --bg: #fdfbf7;
    --font-main: 'Newsreader', Georgia, serif;
    --font-code: 'JetBrains Mono', monospace;
}
@page { size: A4; margin: 25mm; }
body { font-family: var(--font-main); color: var(--text); background: var(--bg); padding: 50px 20px; max-width: 740px; margin: 0 auto; font-size: 18px; line-height: 1.8; }
.doc-header { text-align: center; margin-bottom: 48px; }
.doc-title { font-size: 2.8rem; font-weight: 600; margin-bottom: 12px; color: var(--primary); }
.doc-byline { font-style: italic; color: var(--text-muted); font-size: 1rem; }
h1, h2, h3 { color: var(--primary); font-weight: 600; margin-top: 1.6em; }
table { width: 100%; border-collapse: collapse; margin: 24px 0; }
th, td { padding: 10px 12px; border-bottom: 1px solid #e7e5e4; text-align: left; }
th { border-bottom: 2px solid var(--primary); font-weight: 600; }
.doc-footer { margin-top: 60px; padding-top: 20px; border-top: 1px solid #e7e5e4; font-size: 0.85rem; color: var(--text-muted); text-align: center; }
"#;

const CORPORATE_SLATE_CSS: &str = r#"
:root {
    --primary: #1e3a8a;
    --accent: #3b82f6;
    --text: #1e293b;
    --text-muted: #64748b;
    --bg: #ffffff;
    --card-bg: #f1f5f9;
    --border: #cbd5e1;
    --font-main: 'Inter', -apple-system, sans-serif;
    --font-code: 'JetBrains Mono', monospace;
}
@page { size: A4; margin: 20mm; }
body { font-family: var(--font-main); color: var(--text); padding: 40px; max-width: 860px; margin: 0 auto; line-height: 1.6; }
.doc-header { border-bottom: 3px solid var(--primary); padding-bottom: 20px; margin-bottom: 30px; }
.doc-title { color: var(--primary); font-size: 2.3rem; font-weight: 700; margin: 0 0 8px 0; }
.doc-byline { color: var(--text-muted); font-size: 0.9rem; }
th { background: var(--primary); color: white; padding: 12px 14px; text-align: left; }
td { border-bottom: 1px solid var(--border); padding: 10px 14px; }
tr:nth-child(even) { background-color: var(--card-bg); }
.doc-footer { margin-top: 50px; padding-top: 20px; border-top: 1px solid var(--border); font-size: 0.85rem; color: var(--text-muted); display: flex; justify-content: space-between; }
"#;

const DARK_GLASS_CSS: &str = r#"
:root {
    --primary: #f8fafc;
    --accent: #818cf8;
    --text: #cbd5e1;
    --text-muted: #94a3b8;
    --bg: #0f172a;
    --card-bg: #1e293b;
    --border: #334155;
    --font-main: 'Inter', sans-serif;
    --font-code: 'JetBrains Mono', monospace;
}
@page { size: A4; margin: 20mm; }
body { font-family: var(--font-main); color: var(--text); background: var(--bg); padding: 40px; max-width: 860px; margin: 0 auto; line-height: 1.65; }
.doc-header { border-bottom: 1px solid var(--border); padding-bottom: 24px; margin-bottom: 36px; }
.doc-title { color: var(--primary); font-size: 2.5rem; text-shadow: 0 0 24px rgba(129, 140, 248, 0.4); margin: 0 0 8px 0; }
.doc-byline { color: var(--text-muted); }
h1, h2, h3 { color: var(--primary); }
pre { background: #020617; border: 1px solid var(--border); color: #a5b4fc; padding: 18px; border-radius: 8px; font-family: var(--font-code); overflow-x: auto; }
code { font-family: var(--font-code); background: var(--card-bg); border: 1px solid var(--border); padding: 2px 6px; border-radius: 4px; color: #e2e8f0; }
table { width: 100%; border-collapse: collapse; margin: 24px 0; }
th, td { border: 1px solid var(--border); padding: 12px 14px; text-align: left; }
th { background: var(--card-bg); color: var(--primary); }
.doc-footer { margin-top: 50px; padding-top: 20px; border-top: 1px solid var(--border); font-size: 0.85rem; color: var(--text-muted); display: flex; justify-content: space-between; }
"#;
