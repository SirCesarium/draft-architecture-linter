use crate::define_language;

define_language!(
    Cpp,
    "C/C++",
    extensions: ["c", "cpp", "h", "hpp", "cc", "cxx", "hh", "hxx"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    import_keywords: ["#include"]
);
