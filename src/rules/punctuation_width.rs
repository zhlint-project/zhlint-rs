//! This rule will format each punctuation into the right width options.
//!
//! Options:
//! - half_width_punctuation: string = `()`
//! - full_width_punctuation: string = `，。：；？！“”‘’`
//!
//! Details:
//! - skip half-width punctuations between half-width content without space
//! - skip successive multiple half-width punctuations
//! - skip ' between half-width content

use crate::{
    config::Config,
    parser::{CharKind, CharKindTrait, Cursor},
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // skip non-punctuation situations
    if !cursor.current().is_punctuation() {
        return;
    }
    // skip half-width punctuations between half-width content without space
    if cursor.current().kind() == CharKind::PunctuationHalf
        && cursor.previous().kind() == CharKind::LettersHalf
        && (cursor.next_skip_space().kind() == CharKind::LettersHalf
            || cursor.next_skip_space() == '\0')
    {
        return;
    }
    // skip ' between half-width content
    if cursor.current() == '\''
        && cursor.previous_skip_space().is_half_width()
        && cursor.next_skip_space().is_half_width()
    {
        return;
    }
    // skip successive multiple half-width punctuations
    if cursor.current().kind() == CharKind::PunctuationHalf
        && !cursor.current().is_bracket()
        && !cursor.current().is_quote()
        && cursor.current() != '\''
        && (cursor.previous().kind() == CharKind::PunctuationHalf
            && !cursor.previous().is_bracket()
            && !cursor.previous().is_quote()
            && cursor.previous() != '\''
            || cursor.next().kind() == CharKind::PunctuationHalf
                && !cursor.next().is_bracket()
                && !cursor.next().is_quote()
                && cursor.next() != '\'')
    {
        return;
    }

    if config
        .half_width_punctuation
        .contains(cursor.current().to_half_width())
    {
        cursor.set(cursor.current().to_half_width())
    }
    if config
        .full_width_punctuation
        .contains(cursor.current().to_full_width())
    {
        cursor.set(cursor.current().to_full_width())
    }
    if cursor.current() == '"' && config.full_width_punctuation.contains("“”") {
        cursor.set(if cursor.search_previous(|c| c == '“', |c| c == '”') {
            '”'
        } else {
            '“'
        })
    }
    if cursor.current() == '\'' && config.full_width_punctuation.contains("‘’") {
        cursor.set(if cursor.search_previous(|c| c == '‘', |c| c == '’') {
            '’'
        } else {
            '‘'
        })
    }
}
