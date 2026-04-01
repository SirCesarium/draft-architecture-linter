# 🍬 Contributing to Sweet

Thank you for your interest in improving Sweet! This guide provides technical specifications for contributing to the core engine and adding support for new programming languages.

---

## 🚀 Adding a New Language

Sweet uses the **Strategy Pattern** combined with a declarative macro to handle language-specific rules. Adding support for a new language is now as simple as a single macro call.

### 1. Define the Language Strategy
Create a new file in `src/languages/definitions/go.rs` and use the `define_language!` macro:

```rust
use crate::define_language;

define_language!(
    Go,
    "Go",
    extensions: ["go"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    import_keywords: ["import"]
    // Optional thresholds override:
    // , thresholds: crate::Thresholds { max_lines: 500, ..Default::default() }
);
```

### 2. Registration
To make the language active, register it in these locations:

1.  **Expose Module**: Add `pub mod go;` to `src/languages/definitions/mod.rs`.
2.  **Register in Registry**: Add `Box::new(definitions::go::Go)` to `LanguageRegistry::new()` in `src/languages/mod.rs`.
3.  **Update Config Schema**: 
    - Add the extension (`go`) to the `ThresholdsOverrides` struct in `src/config/thresholds.rs`.
    - Update the `get()` and `extend()` methods in that same file to handle the new field.

---

## 🧪 Quality Standards

We enforce strict engineering standards to maintain Sweet's performance and reliability:

- **Zero-Copy Focus**: Avoid allocating `String` or `Vec` inside the scanner loop. Use byte slices and offsets.
- **Macro-First**: If you find yourself implementing the same trait pattern for multiple structures, use or extend existing macros to eliminate boilerplate.
- **No Panics**: Use `Result` and `Option`. The codebase has a strict `#![deny(clippy::unwrap_used)]`.
- **Pedantic Clippy**: We use `clippy::pedantic`. All contributions must pass without warnings.

### Validation Workflow
Always run the quality gate before submitting:
```bash
./hooks/pre-push
```
This ensures formatting, lints, tests, and self-analysis are perfect.

---

## 📜 License
By contributing, you agree that your contributions will be licensed under the [MIT License](./LICENSE).
