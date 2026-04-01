use crate::define_language;

define_language!(
    Rust,
    "Rust",
    extensions: ["rs"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    import_keywords: ["use", "mod", "extern crate"]
);
