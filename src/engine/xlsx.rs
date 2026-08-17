use crate::templates::DocumentMeta;
use anyhow::Result;
use rust_xlsxwriter::*;
use std::collections::HashMap;

pub fn render_xlsx_bytes(input_text: &str, _meta: &DocumentMeta) -> Result<Vec<u8>> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Define Styling Formats
    let header_format = Format::new()
        .set_bold()
        .set_font_color(Color::RGB(0xFFFFFF))
        .set_background_color(Color::RGB(0x0F172A))
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let cell_format = Format::new().set_border(FormatBorder::Thin);
    let cell_alt_format = Format::new()
        .set_background_color(Color::RGB(0xF8FAFC))
        .set_border(FormatBorder::Thin);

    let input_trimmed = input_text.trim();
    let mut max_col_widths: HashMap<u16, usize> = HashMap::new();

    // Strategy 1: JSON Array
    if input_trimmed.starts_with('[') {
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(input_trimmed) {
            if let Some(arr) = json_val.as_array() {
                if !arr.is_empty() {
                    // Check if array of objects: [ { "a": 1, "b": 2 }, ... ]
                    if let Some(first_obj) = arr[0].as_object() {
                        let keys: Vec<&String> = first_obj.keys().collect();

                        // Write Header Row
                        for (col_idx, key) in keys.iter().enumerate() {
                            let c = col_idx as u16;
                            worksheet.write_string_with_format(0, c, *key, &header_format)?;
                            let cur_max = max_col_widths.entry(c).or_insert(0);
                            *cur_max = (*cur_max).max(key.len());
                        }

                        // Write Data Rows
                        for (row_idx, item) in arr.iter().enumerate() {
                            let r = (row_idx + 1) as u32;
                            let fmt = if row_idx % 2 == 1 {
                                &cell_alt_format
                            } else {
                                &cell_format
                            };

                            if let Some(obj) = item.as_object() {
                                for (col_idx, key) in keys.iter().enumerate() {
                                    let c = col_idx as u16;
                                    let val = obj.get(*key).unwrap_or(&serde_json::Value::Null);

                                    match val {
                                        serde_json::Value::Number(n) => {
                                            if let Some(f) = n.as_f64() {
                                                worksheet.write_number_with_format(r, c, f, fmt)?;
                                                let len = f.to_string().len();
                                                let cur_max = max_col_widths.entry(c).or_insert(0);
                                                *cur_max = (*cur_max).max(len);
                                            }
                                        }
                                        serde_json::Value::Bool(b) => {
                                            worksheet.write_boolean_with_format(r, c, *b, fmt)?;
                                            let cur_max = max_col_widths.entry(c).or_insert(0);
                                            *cur_max = (*cur_max).max(5);
                                        }
                                        serde_json::Value::String(s) => {
                                            worksheet.write_string_with_format(r, c, s, fmt)?;
                                            let cur_max = max_col_widths.entry(c).or_insert(0);
                                            *cur_max = (*cur_max).max(s.len());
                                        }
                                        serde_json::Value::Null => {
                                            worksheet.write_string_with_format(r, c, "", fmt)?;
                                        }
                                        _ => {
                                            let s = val.to_string();
                                            worksheet.write_string_with_format(r, c, &s, fmt)?;
                                            let cur_max = max_col_widths.entry(c).or_insert(0);
                                            *cur_max = (*cur_max).max(s.len());
                                        }
                                    }
                                }
                            }
                        }

                        // Apply Auto-Width to Columns
                        for (col_idx, max_len) in max_col_widths {
                            let width = (max_len + 4).clamp(10, 50) as f64;
                            worksheet.set_column_width(col_idx, width)?;
                        }

                        let buf = workbook.save_to_buffer()?;
                        return Ok(buf);
                    }

                    // Check if array of arrays: [ ["H1", "H2"], ["D1", "D2"] ]
                    if let Some(_first_row) = arr[0].as_array() {
                        for (row_idx, row_val) in arr.iter().enumerate() {
                            let r = row_idx as u32;
                            let is_header = row_idx == 0;
                            let fmt = if is_header {
                                &header_format
                            } else if row_idx % 2 == 1 {
                                &cell_alt_format
                            } else {
                                &cell_format
                            };

                            if let Some(cells) = row_val.as_array() {
                                for (col_idx, cell) in cells.iter().enumerate() {
                                    let c = col_idx as u16;
                                    let s = match cell {
                                        serde_json::Value::String(st) => st.clone(),
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => cell.to_string(),
                                    };

                                    if let Ok(num) = s.parse::<f64>() {
                                        if !is_header {
                                            worksheet.write_number_with_format(r, c, num, fmt)?;
                                        } else {
                                            worksheet.write_string_with_format(r, c, &s, fmt)?;
                                        }
                                    } else {
                                        worksheet.write_string_with_format(r, c, &s, fmt)?;
                                    }

                                    let cur_max = max_col_widths.entry(c).or_insert(0);
                                    *cur_max = (*cur_max).max(s.len());
                                }
                            }
                        }

                        for (col_idx, max_len) in max_col_widths {
                            let width = (max_len + 4).clamp(10, 50) as f64;
                            worksheet.set_column_width(col_idx, width)?;
                        }

                        let buf = workbook.save_to_buffer()?;
                        return Ok(buf);
                    }
                }
            }
        }
    }

    // Strategy 2: Parse Markdown Table, CSV, or TSV line-by-line
    let mut row_idx = 0u32;
    for line in input_text.lines() {
        let line = line.trim();
        // Skip empty lines or Markdown table separator rows (e.g. |---|---| or | :--- | :--- |)
        if line.is_empty()
            || line.starts_with("|---")
            || line.starts_with("| ---")
            || line.starts_with("|:---")
            || line.starts_with("| :---")
            || line
                .chars()
                .all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
        {
            continue;
        }

        let cols: Vec<&str> = if line.contains('|') {
            line.split('|')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim())
                .collect()
        } else if line.contains('\t') {
            line.split('\t').map(|s| s.trim()).collect()
        } else {
            line.split(',').map(|s| s.trim()).collect()
        };

        if !cols.is_empty() {
            let is_header = row_idx == 0;
            let fmt = if is_header {
                &header_format
            } else if row_idx % 2 == 1 {
                &cell_alt_format
            } else {
                &cell_format
            };

            for (col_idx, col_val) in cols.iter().enumerate() {
                let c = col_idx as u16;
                if let Ok(num) = col_val.parse::<f64>() {
                    if !is_header {
                        worksheet.write_number_with_format(row_idx, c, num, fmt)?;
                    } else {
                        worksheet.write_string_with_format(row_idx, c, *col_val, fmt)?;
                    }
                } else {
                    worksheet.write_string_with_format(row_idx, c, *col_val, fmt)?;
                }

                let cur_max = max_col_widths.entry(c).or_insert(0);
                *cur_max = (*cur_max).max(col_val.len());
            }
            row_idx += 1;
        }
    }

    // Set auto-width
    for (col_idx, max_len) in max_col_widths {
        let width = (max_len + 4).clamp(10, 50) as f64;
        worksheet.set_column_width(col_idx, width)?;
    }

    let buf = workbook.save_to_buffer()?;
    Ok(buf)
}
