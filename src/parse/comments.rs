pub const COMMENT_START: &'static str = "#(";
pub const COMMENT_END: &'static str = ")#";

pub fn remove_non_documenting_comments(mut code: String) -> String {
    while let Some(start) = code.find(COMMENT_START) {
        let before = code[..start + COMMENT_START.len()].to_string();
        let after = remove_non_documenting_comments(code[start + COMMENT_START.len()..].to_string());

        code = before + &after;

        if let Some(end) = code[start + COMMENT_START.len()..].find(COMMENT_END) {
            code.replace_range(start..end + COMMENT_END.len() * 2 + start, "")
        } else {
            panic!("unterminated comment")
        }
    }

    code
}
