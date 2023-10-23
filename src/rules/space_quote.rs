//! This rule is checking spaces besides quotes.
//!
//! Options
//! - no_space_inside_quote: bool
//! - space_outside_half_quote: Option<bool>
//! - no_space_outside_full_quote: bool
//!
//! Details:
//! - no_space_inside_quote:
//!   - left-quote x right-quote
//!   - content/punctuation/right-quote/right-quote/code/unknown/container x right-quote
//!   - left-quote x content/punctuation/left-quote/left-quote/code/unknown/container
//! - space_outside_half_quote:
//!   - right-half-quote x left-half-quote
//!   - content/code x left-half-quote
//!   - right-half-quote x content/code
//! - no_space_outside_full_quote:
//!   - right-full-quote x left-full-quote
//!   - content/code x left-full-quote
//!   - right-full-quote x content/code

use crate::{
    config::Config,
    parser::{CharKindTrait, Cursor},
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // 1. no space inside quote
    let previous_is_left_quote = cursor.previous().is_left_quote()
        || cursor.previous().is_half_width_quote()
            && cursor.count_previous(|c| c == cursor.previous()) % 2 == 1;
    let next_is_right_quote = cursor.next().is_right_quote()
        || cursor.next().is_half_width_quote()
            && cursor.count_previous(|c| c == cursor.next()) % 2 == 1;
    if config.no_space_inside_quote
        && cursor.current().is_whitespace()
        && ((previous_is_left_quote && cursor.next().is_letters())
            || (cursor.previous().is_letters() && next_is_right_quote))
    {
        cursor.remove()
    }

    // 2. spaces outside half quote
    match config.space_outside_half_quote {
        Some(true) => {
            if (cursor.previous().is_letters()
                && cursor.current().is_half_width_quote()
                && cursor.count_previous(|c| c == cursor.current()) % 2 == 0)
                || (cursor.previous().is_half_width_quote()
                    && cursor.count_previous(|c| c == cursor.previous()) % 2 == 0
                    && cursor.current().is_letters())
            {
                cursor.add(' ')
            }
        }
        Some(false) => {
            let next_is_left_quote = cursor.next().is_half_width_quote()
                && cursor.count_previous(|c| c == cursor.next()) % 2 == 0;
            let previous_is_right_quote = cursor.previous().is_half_width_quote()
                && cursor.count_previous(|c| c == cursor.previous()) % 2 == 0;
            if cursor.current().is_whitespace()
                && ((cursor.previous().is_letters() && next_is_left_quote)
                    || (previous_is_right_quote && cursor.next().is_letters()))
            {
                cursor.remove()
            }
        }
        None => (),
    }

    // 3. no space outside full quote
    if config.no_space_outside_full_quote
        && cursor.current().is_whitespace()
        && ((cursor.previous().is_letters() && cursor.next().is_left_quote())
            || (cursor.previous().is_right_quote() && cursor.next().is_letters()))
    {
        cursor.remove()
    }
}
