pub fn html_entities(dest: &mut String, src: &str) {
    for c in src.chars() {
        match c {
            '&' => dest.push_str("&amp;"),
            '<' => dest.push_str("&lt;"),
            '>' => dest.push_str("&gt;"),
            '"' => dest.push_str("&quot;"),
            '\'' => dest.push_str("&#x27;"),
            '/' => dest.push_str("&#x2f;"),
            '`' => dest.push_str("&#x96;"),
            _ => dest.push(c),
        }
    }
}

pub fn quoted_shell_argument(dest: &mut String, src: &str) {
    // Do same as python and bash: enclouse in single quotes
    // and escape a single quote.
    // Known quirk: zsh converges double backslash even in single-quotes
    dest.push('\'');
    for c in src.chars() {
        match c {
            '\'' => dest.push_str(r#"'"'"'"#),
            _ => dest.push(c),
        }
    }
    dest.push('\'');
}
