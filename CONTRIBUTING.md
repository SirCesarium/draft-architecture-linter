# 🍬 Contributing to Sweet

Thank you for your interest in improving Sweet! This guide provides technical specifications for contributing to the core engine and adding support for new programming languages.

---

## 🏗️ Project Architecture (Single-Pass Zero-Copy)

Sweet is built for industrial-grade performance. Unlike traditional analyzers that read files multiple times, Sweet uses a **Unified Single-Pass Engine**:

1.  **Discovery**: The `ignore` walker finds files respecting `.gitignore`.
2.  **Memory Mapping**: Files are mapped into memory using `memmap2` for zero-copy access.
3.  **Unified Scan**: The `scanner::scan` function traverses the byte buffer **exactly once**.
    - It tracks nesting depth via indentation.
    - It identifies imports using language-specific keywords.
    - It strips comments on-the-fly to produce a "clean" buffer for repetition analysis.
4.  **Repetition Analysis**: A specialized module computes hashes of the clean buffer to find project-wide clones.

---

## 🚀 Adding a New Language

Sweet uses the **Strategy Pattern** to handle language-specific rules. Adding a language is purely declarative.

### 1. Define the Language Strategy
Create a new file in `src/languages/definitions/go.rs`:

```rust
use crate::languages::Language;

pub struct Go;

impl Language for Go {
    fn name(&self) -> &'static str { "Go" }
    fn extensions(&self) -> &'static [&'static str] { &["go"] }
    fn line_comment(&self) -> Option<&'static str> { Some("//") }
    fn block_comment(&self) -> Option<(&'static str, &'static str)> { Some(("/*", "*/")) }
    fn import_keywords(&self) -> &'static [&'static str] { 
        &["import", "import ("] 
    }
    
    // Optional: Number of spaces per indent level (default: 4)
    fn indent_size(&self) -> usize { 8 }

    // Optional: Default thresholds specifically for this language
    fn default_thresholds(&self) -> crate::Thresholds {
        crate::Thresholds {
            max_lines: 400,
            max_imports: 20,
            ..Default::default()
        }
    }
}
```

### 2. Registration
To make the language active, you must register it in three places:

1.  **Expose Module**: Add `pub mod go;` to `src/languages/definitions/mod.rs`.
2.  **Register in Engine**: Add `Box::new(definitions::go::Go)` to `LanguageRegistry::new()` in `src/languages/mod.rs`.
3.  **Update Config Schema**: 
    - Add the extension (`go`) to the `ThresholdsOverrides` struct in `src/config/thresholds.rs`.
    - Update the `get()` and `extend()` methods in that same file to handle the new field.

### 3. VS Code Integration
1.  **`editors/vscode/package.json`**: Add the language ID to the `languages` contribution point.
2.  **`editors/vscode/src/extension.ts`**: Add the language ID to the `supportedLanguages` array to enable LSP features.

---

## 🧪 Quality Standards

We enforce strict engineering standards to maintain Sweet's performance and reliability:

- **Zero-Copy First**: Avoid allocating `String` or `Vec` inside the scanner loop. Use byte slices (`&[u8]`) and offsets.
- **No Panics**: Use `Result` and `Option`. The codebase has a strict `#![deny(clippy::unwrap_used)]`.
- **Pedantic Clippy**: We use `clippy::pedantic`. All contributions must pass without warnings.
- **Performance Regression**: If your change affects the core loop, run `cargo bench` to ensure no performance loss.

### Validation Workflow
Run the pre-push hook before submitting a PR:
```bash
./hooks/pre-push
```
This ensures formatting, lints, and tests are perfect.

---

## 📜 License
By contributing, you agree that your contributions will be licensed under the [MIT License](./LICENSE).
