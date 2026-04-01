use crate::define_language;

define_language!(
    CSharp,
    "C#",
    extensions: ["cs"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    import_keywords: ["using", "namespace"]
);
