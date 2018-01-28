use std::collections::HashMap;

use preparser::Syntax;
use validators::Filter;
use {Options};

impl Options {
    /// Create options with all defaults values
    pub fn new() -> Options {
        Options {
            syntax: Syntax::Plain,
            new_line_at_eof: None,
            curly: false,
            square: false,
            round: false,
            default_filter: Filter::NoFilter,
            filters: HashMap::new(),
        }
    }
    /// Enables `oneline` syntax by default
    ///
    /// This is equivalent as `## syntax: oneline` in a template. But
    /// template author can still override the syntax.
    pub fn syntax_oneline(&mut self) -> &mut Self {
        self.syntax = Syntax::Oneline;
        self
    }
    /// Enables `indent` syntax by default
    ///
    /// This is equivalent as `## syntax: indent` in a template. But
    /// template author can still override the syntax.
    pub fn syntax_indent(&mut self) -> &mut Self {
        self.syntax = Syntax::Indent;
        self
    }
}
