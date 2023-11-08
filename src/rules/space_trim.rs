//! This rule is trimming spaces of the whole string.

use pulldown_cmark::Event;

use crate::{
    config::Config,
    parser::{TextCursor, Token},
    Context,
};

pub fn rule(_ctx: &Context, cursor: &mut TextCursor, config: &Config) {
    if config.rules.no_space_inside_wrapper
        && cursor.current().is_whitespace()
        && (matches!(cursor.prev(), Token::Event(Event::Start(_)))
            || matches!(cursor.next(), Token::Event(Event::End(_))))
    {
        cursor.delete();
    }
}
