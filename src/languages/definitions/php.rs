use crate::define_language;

define_language!(
    PHP,
    "PHP",
    extensions: ["php"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    import_keywords: ["use", "require", "include", "namespace"]
);
