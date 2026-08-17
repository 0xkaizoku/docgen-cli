# 🎨 `docgen-cli` Themes & Design Presets

`docgen-cli` comes out of the box with 5 professionally designed typography and layout themes. Each theme is curated with harmonious color schemes, modern typography, responsive layout containers, and print-ready page settings (`@page`).

---

## 🎨 Built-in Theme Catalog

### 1. `modern-executive` *(Default)*
- **Typography**: `Inter`, clean sans-serif typography.
- **Palette**: Deep slate (`#0F172A`), royal blue accent (`#2563EB`), muted grey secondary (`#64748B`).
- **Aesthetic**: Polished, modern enterprise feel. Features subtle borders, shaded header cards, and clean table formatting.
- **Best For**: Executive summaries, business memos, product specifications, quarterly reviews.

### 2. `tech-spec`
- **Typography**: `JetBrains Mono` combined with `Inter`.
- **Palette**: Charcoal (`#09090B`), cyan accent (`#0284C7`), dark code containers (`#18181B`).
- **Aesthetic**: Monospace-heavy, developer-centric layout. Features distinctive left-accent header bars and high-contrast code highlighting.
- **Best For**: RFCs, architecture decision records (ADRs), API references, infrastructure playbooks.

### 3. `minimal-paper`
- **Typography**: `Newsreader` / Georgia editorial serif typography.
- **Palette**: Warm cream background (`#FDFBF7`), ink black text (`#1C1917`), stone accents (`#78716C`).
- **Aesthetic**: Elegant, bookish, distraction-free reading experience with generous line height and centered titling.
- **Best For**: Research papers, long-form essays, technical whitepapers, documentation articles.

### 4. `corporate-slate`
- **Typography**: Crisp enterprise sans-serif.
- **Palette**: Navy blue header background (`#1E3A8A`), sky blue accent (`#3B82F6`), slate grey text (`#1E293B`).
- **Aesthetic**: Formal corporate layout with solid navy table headers, structured borders, and high-density information layout.
- **Best For**: Financial spreadsheets, compliance audits, invoices, formal business contracts.

### 5. `dark-glass`
- **Typography**: `Inter` with glowing accent styling.
- **Palette**: Midnight slate background (`#0F172A`), indigo accent (`#818CF8`), glow effects, dark cards (`#1E293B`).
- **Aesthetic**: Modern glassmorphism dark mode with vibrant code syntax highlighting and soft glowing text shadows.
- **Best For**: Modern web documentation, dark-theme presentations, developer previews.

---

## 🖨️ Print & PDF Styling

All HTML templates include `@page` directives optimized for standard A4 printing and PDF generation:

```css
@page {
    size: A4;
    margin: 20mm;
}
```

Headers, tables, and code blocks automatically respect page-break rules (`page-break-inside: avoid`) to prevent awkward splits across printed pages.
