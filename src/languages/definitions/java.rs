use crate::define_language;

define_language!(
    Java,
    "Java",
    extensions: ["java"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    import_keywords: ["import", "package"]
);
