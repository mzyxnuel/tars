/// All component CSS bundled into one string. Mount once, near the root
/// of the app: `style { {STYLES} }`. Variables are CSS custom properties
/// so apps can override them by adding their own `:root { --tars-... }`.
pub const STYLES: &str = r#"
:root {
    --tars-bg: #0b1020;
    --tars-surface: #11182c;
    --tars-surface-2: #1a2240;
    --tars-border: #28315a;
    --tars-text: #e6ecff;
    --tars-text-dim: #a4adcc;
    --tars-primary: #5b8cff;
    --tars-primary-strong: #3d6ef5;
    --tars-danger: #ef4f6e;
    --tars-success: #2dd4a4;
    --tars-warning: #fbbf24;
    --tars-radius: 10px;
    --tars-shadow: 0 8px 24px rgba(0,0,0,0.25);
    --tars-mono: ui-monospace, SFMono-Regular, Menlo, monospace;
    --tars-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
}

* { box-sizing: border-box; }

html, body {
    margin: 0;
    padding: 0;
    background: var(--tars-bg);
    color: var(--tars-text);
    font-family: var(--tars-sans);
    font-size: 15px;
    line-height: 1.5;
}

a { color: var(--tars-primary); text-decoration: none; }
a:hover { text-decoration: underline; }

/* Container */
.tars-container {
    max-width: 980px;
    margin: 0 auto;
    padding: 24px;
}

/* Page */
.tars-page { padding: 24px 0; }
.tars-page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 20px;
    gap: 12px;
}
.tars-page-actions { display: flex; gap: 8px; }

/* Headings */
.tars-h1 { font-size: 28px; font-weight: 700; margin: 0; }
.tars-h2 { font-size: 22px; font-weight: 600; margin: 0; }
.tars-h3 { font-size: 18px; font-weight: 600; margin: 0; }

/* Buttons */
.tars-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 8px 14px;
    border-radius: var(--tars-radius);
    border: 1px solid transparent;
    font-weight: 600;
    font-size: 14px;
    cursor: pointer;
    transition: background 120ms, transform 80ms, border-color 120ms;
    user-select: none;
    text-decoration: none;
}
.tars-btn:active { transform: translateY(1px); }
.tars-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.tars-btn-primary { background: var(--tars-primary); color: white; }
.tars-btn-primary:hover:not(:disabled) { background: var(--tars-primary-strong); }
.tars-btn-secondary {
    background: transparent;
    color: var(--tars-text);
    border-color: var(--tars-border);
}
.tars-btn-secondary:hover:not(:disabled) { background: var(--tars-surface-2); }
.tars-btn-danger { background: var(--tars-danger); color: white; }
.tars-btn-ghost {
    background: transparent;
    color: var(--tars-text-dim);
    padding: 6px 10px;
}
.tars-btn-ghost:hover:not(:disabled) { color: var(--tars-text); background: var(--tars-surface-2); }

/* Cards */
.tars-card {
    background: var(--tars-surface);
    border: 1px solid var(--tars-border);
    border-radius: var(--tars-radius);
    box-shadow: var(--tars-shadow);
    overflow: hidden;
}
.tars-card-header {
    padding: 14px 18px;
    border-bottom: 1px solid var(--tars-border);
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.tars-card-body { padding: 18px; }

/* Forms */
.tars-form-group { display: flex; flex-direction: column; gap: 14px; }
.tars-form-field { display: flex; flex-direction: column; gap: 6px; }
.tars-label { font-size: 13px; color: var(--tars-text-dim); font-weight: 500; }
.tars-input, .tars-textarea {
    width: 100%;
    padding: 9px 12px;
    background: var(--tars-surface-2);
    color: var(--tars-text);
    border: 1px solid var(--tars-border);
    border-radius: var(--tars-radius);
    font-size: 14px;
    font-family: inherit;
    transition: border-color 120ms, box-shadow 120ms;
}
.tars-input:focus, .tars-textarea:focus {
    outline: none;
    border-color: var(--tars-primary);
    box-shadow: 0 0 0 3px rgba(91, 140, 255, 0.18);
}
.tars-textarea { min-height: 100px; resize: vertical; }
.tars-error {
    color: var(--tars-danger);
    font-size: 12px;
    margin-top: 2px;
}

/* Alerts */
.tars-alert {
    padding: 12px 14px;
    border-radius: var(--tars-radius);
    border: 1px solid transparent;
    font-size: 14px;
}
.tars-alert-info { background: rgba(91,140,255,0.12); border-color: rgba(91,140,255,0.4); color: #c8d8ff; }
.tars-alert-success { background: rgba(45,212,164,0.10); border-color: rgba(45,212,164,0.4); color: #b8f5dc; }
.tars-alert-error { background: rgba(239,79,110,0.10); border-color: rgba(239,79,110,0.4); color: #ffcad5; }
.tars-alert-warning { background: rgba(251,191,36,0.10); border-color: rgba(251,191,36,0.4); color: #ffe7a8; }

/* Tables */
.tars-table {
    width: 100%;
    border-collapse: collapse;
    background: var(--tars-surface);
    border: 1px solid var(--tars-border);
    border-radius: var(--tars-radius);
    overflow: hidden;
}
.tars-table th, .tars-table td {
    padding: 12px 14px;
    text-align: left;
    border-bottom: 1px solid var(--tars-border);
}
.tars-table th {
    background: var(--tars-surface-2);
    font-size: 12px;
    font-weight: 600;
    color: var(--tars-text-dim);
    text-transform: uppercase;
    letter-spacing: 0.04em;
}
.tars-table tr:last-child td { border-bottom: none; }
.tars-table tr:hover td { background: rgba(91,140,255,0.04); }

/* Spinner */
.tars-spinner {
    display: inline-block;
    width: 18px;
    height: 18px;
    border: 2px solid var(--tars-border);
    border-top-color: var(--tars-primary);
    border-radius: 50%;
    animation: tars-spin 0.7s linear infinite;
}
@keyframes tars-spin { to { transform: rotate(360deg); } }

/* App nav */
.tars-app { min-height: 100vh; }
.tars-nav {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 14px 24px;
    background: var(--tars-surface);
    border-bottom: 1px solid var(--tars-border);
}
.tars-nav-brand { font-weight: 700; letter-spacing: 0.04em; }
.tars-nav a { color: var(--tars-text-dim); font-weight: 500; }
.tars-nav a:hover { color: var(--tars-text); text-decoration: none; }
.tars-nav a.active { color: var(--tars-primary); }

/* Utilities */
.tars-muted { color: var(--tars-text-dim); }
.tars-row { display: flex; gap: 12px; align-items: center; }
.tars-stack { display: flex; flex-direction: column; gap: 12px; }
.tars-grid {
    display: grid;
    gap: 16px;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
}
"#;
