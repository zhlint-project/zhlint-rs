//! This rule will decide whether to keep a space outside inline code with
//! content like:
//! - xxx `foo` xxx
//! in markdown.
//!
//! Options:
//! - spaceOutsideCode: boolean | undefined
//!   - `true`: keep one space outside (default)
//!   - `false`: no space outside
//!   - `undefined`: do nothing, just keep the original format
//!
//! Details:
//! - code x code
//! - content x code
//! - code x content

use pulldown_cmark::Event;

use crate::{
    char_kind::CharKindTrait,
    config::Config,
    parser::{TextCursor, Token},
    Context,
};

pub fn rule(_ctx: &Context, cursor: &mut TextCursor, config: &Config) {
    match config.rules.space_outside_code {
        Some(true) => {
            if cursor.current().is_letters()
                && matches!(cursor.next(), Token::Event(Event::Code(_)))
            {
                cursor.add_next(' ');
            }
            if cursor.current().is_letters()
                && matches!(cursor.prev(), Token::Event(Event::Code(_)))
            {
                cursor.add_prev(' ');
            }
        }
        Some(false) => {
            if cursor.prev().is_letters()
                && cursor.current().is_whitespace()
                && matches!(cursor.next(), Token::Event(Event::Code(_)))
            {
                cursor.delete();
            }
            if matches!(cursor.prev(), Token::Event(Event::Code(_)))
                && cursor.current().is_whitespace()
                && cursor.next().is_letters()
            {
                cursor.delete();
            }
        }
        None => (),
    }
}
