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

use crate::{
    config::Config,
    parser::{CharKind, CharKindTrait, Cursor},
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // 1. no space before punctuation
    if config.no_space_before_punctuation
        && cursor.current().is_whitespace()
        && cursor.next().is_common_punctuation()
    {
        cursor.remove()
    }

    // 2. space after half width punctuation
    match config.space_after_half_width_punctuation {
        Some(true) => {
            if cursor.previous().kind() == CharKind::PunctuationHalf
                && cursor.previous().is_common_punctuation()
                && !cursor.current().is_whitespace()
                // skip successive multiple half-width punctuations
                && !(cursor.current().kind() == CharKind::PunctuationHalf
                    && cursor.current().is_common_punctuation())
                && cursor.previous_two().kind() != CharKind::PunctuationHalf
                // skip half-width punctuations between half-width content without space
                && !(cursor.current().kind() == CharKind::LettersHalf
                    && cursor.previous_two().kind() == CharKind::LettersHalf)
            {
                cursor.add(' ')
            }
        }
        Some(false) => {
            if cursor.previous().kind() == CharKind::PunctuationHalf
                && cursor.previous().is_common_punctuation()
                && cursor.current().is_whitespace()
            {
                cursor.remove()
            }
        }
        None => (),
    }

    // 3. no space after full width punctuation
    if config.no_space_after_full_width_punctuation
        && cursor.previous().kind() == CharKind::PunctuationFull
        && cursor.previous().is_common_punctuation()
        && cursor.current().is_whitespace()
    {
        cursor.remove()
    }
}
