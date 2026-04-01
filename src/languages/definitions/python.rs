use crate::define_language;

define_language!(
    Python,
    "Python",
    extensions: ["py"],
    line_comment: Some("#"),
    block_comment: Some(("\"\"\"", "\"\"\"")),
    import_keywords: ["import", "from"]
);
