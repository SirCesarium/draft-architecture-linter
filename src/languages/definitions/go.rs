use crate::define_language;

define_language!(
    Go,
    "Go",
    extensions: ["go"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    import_keywords: ["import"]
);
