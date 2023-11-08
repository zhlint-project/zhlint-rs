//! This rule is checking spaces besides normal punctuations.
//! Usually, for full-width punctuations, we don't need any spaces around.
//! For half-width punctuations, we need a space after that.
//!
//! Options
//! - no_space_before_punctuation: bool
//!   - `true`: remove spaces before a half-width punctuation (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//! - space_after_half_width_punctuation: Option<bool>
//!   - `true`: ensure one space after a half-width punctuation (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//! - no_space_after_full_width_punctuation: bool
//!   - `true`: remove spaces around a full-width punctuation (default)
//!   - `false` or `undefined`: do nothing, just keep the original format
//!
//! Details:
//! - no_space_before_punctuation:
//!   content/right-quote/right-bracket/code x punctuation
//! - space_after_half_width_punctuation:
//!   half x content/left-quote/left-bracket/code
//! - no_space_after_full_width_punctuation:
//!   full x content/left-quote/left-bracket/code
//!
//! - skip half-width punctuations between half-width content without space
//! - skip successive multiple half-width punctuations

use pulldown_cmark::Event;

use crate::{
    char_kind::{CharKind, CharKindTrait},
    config::Config,
    parser::{TextCursor, Token},
    Context,
};

pub fn rule(_ctx: &Context, cursor: &mut TextCursor, config: &Config) {
    // 1. no space before punctuation
    if config.rules.no_space_before_punctuation
        && !matches!(cursor.prev(), Token::Event(Event::Code(_)))
        && cursor.current().is_whitespace()
        && cursor.next().is_common_punctuation()
    {
        cursor.delete();
    }

    // 2. space after half width punctuation
    match config.rules.space_after_half_width_punctuation {
        Some(true) => {
            if cursor.current().kind() == CharKind::PunctuationHalf
                && cursor.current().is_common_punctuation()
                && cursor.next().kind() != CharKind::Space
                // skip successive multiple half-width punctuations
                && !(cursor.next().kind() == CharKind::PunctuationHalf
                    && cursor.next().is_common_punctuation())
                // skip half-width punctuations between half-width content without space
                && !(cursor.next().kind() == CharKind::LettersHalf
                    && cursor.prev().kind() == CharKind::LettersHalf)
            {
                cursor.add_next(' ');
            }
        }
        Some(false) => {
            if cursor.prev().kind() == CharKind::PunctuationHalf
                && cursor.prev().is_common_punctuation()
                && cursor.current().is_whitespace()
            {
                cursor.delete();
            }
        }
        None => (),
    }

    // 3. no space after full width punctuation
    if config.rules.no_space_after_full_width_punctuation
        && cursor.prev().kind() == CharKind::PunctuationFull
        && cursor.prev().is_common_punctuation()
        && cursor.current().is_whitespace()
    {
        cursor.delete();
    }
}
