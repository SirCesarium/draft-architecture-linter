use crate::define_language;

define_language!(
    GDScript,
    "GDScript",
    extensions: ["gd"],
    line_comment: Some("#"),
    block_comment: None,
    import_keywords: ["extends", "class_name", "preload", "load"]
);
