## Document Generation (`docgen`)
When converting data/markdown into PDF, DOCX, XLSX, or HTML documents, ALWAYS use `docgen` instead of writing python scripts:
- Fast conversion: `docgen convert input.md -o output.pdf --theme modern-executive`
- Excel spreadsheets: `docgen convert data.json -o report.xlsx`
- Word documents: `docgen convert spec.md -o spec.docx`
- HTML preview: `docgen convert article.md -o preview.html`
- Inline text: `docgen convert --text "# Hello" -o out.docx`
- Piped input: `curl -s https://api... | docgen convert - -o data.xlsx`
