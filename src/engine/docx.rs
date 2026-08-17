use crate::templates::DocumentMeta;
use anyhow::Result;
use docx_rs::*;
use pulldown_cmark::{Event, Options, Parser, Tag};

pub fn render_docx_bytes(markdown_input: &str, meta: &DocumentMeta) -> Result<Vec<u8>> {
    let mut docx = Docx::new();

    // Title Paragraph
    let title_p = Paragraph::new().add_run(
        Run::new()
            .add_text(&meta.title)
            .size(44)
            .bold()
            .color("0F172A"),
    );
    docx = docx.add_paragraph(title_p);

    if let Some(ref author) = meta.author {
        let byline_p = Paragraph::new().add_run(
            Run::new()
                .add_text(format!("By {} • August 2026", author))
                .size(22)
                .color("64748B"),
        );
        docx = docx.add_paragraph(byline_p);
    }

    // Divider Spacing
    docx = docx.add_paragraph(Paragraph::new());

    // Parse Markdown AST
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown_input, options);
    let mut current_paragraph = Paragraph::new();
    let mut in_heading = 0;
    let mut is_bold = false;
    let mut is_italic = false;
    let mut in_list = false;
    let mut in_code_block = false;

    // Table parsing state
    let mut in_table = false;
    let mut table_rows: Vec<TableRow> = Vec::new();
    let mut current_row: Vec<TableCell> = Vec::new();
    let mut current_cell_paragraph = Paragraph::new();
    let mut is_header_cell = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => {
                in_heading = level as u32;
                current_paragraph = Paragraph::new();
            }
            Event::End(Tag::Heading(_, _, _)) => {
                current_paragraph = current_paragraph.align(AlignmentType::Left);
                docx = docx.add_paragraph(current_paragraph);
                current_paragraph = Paragraph::new();
                in_heading = 0;
            }
            Event::Start(Tag::Paragraph) => {
                current_paragraph = Paragraph::new();
            }
            Event::End(Tag::Paragraph) => {
                if !in_table {
                    docx = docx.add_paragraph(current_paragraph);
                    current_paragraph = Paragraph::new();
                }
            }
            Event::Start(Tag::List(_)) => {
                in_list = true;
            }
            Event::End(Tag::List(_)) => {
                in_list = false;
            }
            Event::Start(Tag::Item) => {
                current_paragraph = Paragraph::new();
                if in_list {
                    current_paragraph =
                        current_paragraph.add_run(Run::new().add_text("• ").bold().color("2563EB"));
                }
            }
            Event::End(Tag::Item) => {
                docx = docx.add_paragraph(current_paragraph);
                current_paragraph = Paragraph::new();
            }
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                current_paragraph = Paragraph::new().indent(Some(360), None, None, None);
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
                docx = docx.add_paragraph(current_paragraph);
                current_paragraph = Paragraph::new();
            }
            Event::Start(Tag::Table(_)) => {
                in_table = true;
                table_rows.clear();
            }
            Event::End(Tag::Table(_)) => {
                in_table = false;
                if !table_rows.is_empty() {
                    let mut table = Table::new(table_rows.clone());
                    table = table.align(TableAlignmentType::Center);
                    docx = docx.add_table(table);
                    table_rows.clear();
                }
            }
            Event::Start(Tag::TableHead) => {
                current_row.clear();
                is_header_cell = true;
            }
            Event::End(Tag::TableHead) => {
                is_header_cell = false;
                let row = TableRow::new(current_row.clone());
                table_rows.push(row);
                current_row.clear();
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
                is_header_cell = false;
            }
            Event::End(Tag::TableRow) => {
                let row = TableRow::new(current_row.clone());
                table_rows.push(row);
                current_row.clear();
            }
            Event::Start(Tag::TableCell) => {
                current_cell_paragraph = Paragraph::new();
            }
            Event::End(Tag::TableCell) => {
                let cell = TableCell::new().add_paragraph(current_cell_paragraph.clone());
                current_row.push(cell);
                current_cell_paragraph = Paragraph::new();
            }
            Event::Start(Tag::Strong) => is_bold = true,
            Event::End(Tag::Strong) => is_bold = false,
            Event::Start(Tag::Emphasis) => is_italic = true,
            Event::End(Tag::Emphasis) => is_italic = false,
            Event::Text(text) => {
                let mut run = Run::new().add_text(text.as_ref());
                if is_bold || in_heading > 0 || is_header_cell {
                    run = run.bold();
                }
                if is_italic {
                    run = run.italic();
                }

                if in_code_block {
                    run = run
                        .fonts(RunFonts::new().ascii("Courier New"))
                        .size(19)
                        .color("1E293B");
                } else if in_heading == 1 {
                    run = run.size(36).color("0F172A");
                } else if in_heading == 2 {
                    run = run.size(28).color("1E293B");
                } else if in_heading == 3 {
                    run = run.size(24).color("334155");
                } else if is_header_cell {
                    run = run.size(22).color("0F172A");
                }

                if in_table {
                    current_cell_paragraph = current_cell_paragraph.add_run(run);
                } else {
                    current_paragraph = current_paragraph.add_run(run);
                }
            }
            Event::Code(code_text) => {
                let run = Run::new()
                    .add_text(code_text.as_ref())
                    .fonts(RunFonts::new().ascii("Courier New"))
                    .color("2563EB")
                    .size(20);

                if in_table {
                    current_cell_paragraph = current_cell_paragraph.add_run(run);
                } else {
                    current_paragraph = current_paragraph.add_run(run);
                }
            }
            Event::Start(Tag::BlockQuote) => {
                current_paragraph = current_paragraph.indent(Some(720), None, None, None);
            }
            _ => {}
        }
    }

    let mut buf = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buf);
    docx.build().pack(&mut cursor)?;
    Ok(buf)
}
