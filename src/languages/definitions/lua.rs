use crate::define_language;

define_language!(
    Lua,
    "Lua",
    extensions: ["lua"],
    line_comment: Some("--"),
    block_comment: Some(("--[[", "]]")),
    import_keywords: ["require"]
);
