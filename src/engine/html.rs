use crate::templates::{wrap_html, DocumentMeta};
use anyhow::Result;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn render_html_string(markdown_input: &str, meta: &DocumentMeta) -> Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown_input, options);

    // Lazy/cached syntax and theme set
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syn_theme = &ts.themes["base16-ocean.dark"];

    let mut custom_events = Vec::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                code_content.clear();
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
                let syntax = ss
                    .find_syntax_by_token(&code_lang)
                    .unwrap_or_else(|| ss.find_syntax_plain_text());

                let highlighted =
                    highlighted_html_for_string(&code_content, &ss, syntax, syn_theme)
                        .unwrap_or_else(|_| {
                            format!("<pre><code>{}</code></pre>", html_escape(&code_content))
                        });

                custom_events.push(Event::Html(highlighted.into()));
            }
            Event::Text(ref text) if in_code_block => {
                code_content.push_str(text);
            }
            _ => {
                custom_events.push(event);
            }
        }
    }

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, custom_events.into_iter());

    let full_document = wrap_html(&html_output, meta);
    Ok(full_document)
}

pub fn render_html_bytes(markdown_input: &str, meta: &DocumentMeta) -> Result<Vec<u8>> {
    let html_str = render_html_string(markdown_input, meta)?;
    Ok(html_str.into_bytes())
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
