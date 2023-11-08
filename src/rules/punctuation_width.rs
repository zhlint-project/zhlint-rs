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
    char_kind::{CharKind, CharKindTrait},
    config::Config,
    parser::{TextCursor, Token},
    Context,
};

pub fn rule(ctx: &Context, cursor: &mut TextCursor, config: &Config) {
    // skip non-punctuation situations
    if !cursor.current().is_punctuation() {
        return;
    }
    // skip half-width punctuations between half-width content without space
    if cursor.current().kind() == CharKind::PunctuationHalf
        && cursor.prev_skip_space().kind() == CharKind::LettersHalf
        && (cursor.next_skip_space().kind() == CharKind::LettersHalf
            || !matches!(cursor.next_skip_space(), Token::Char(_)))
    {
        return;
    }
    // skip ' between half-width content
    if cursor.current() == '\''
        && cursor.prev_skip_space().is_half_width()
        && cursor.next_skip_space().is_half_width()
    {
        return;
    }
    // skip successive multiple half-width punctuations
    if cursor.current().kind() == CharKind::PunctuationHalf
        && !cursor.current().is_bracket()
        && !cursor.current().is_quote()
        && cursor.current() != '\''
        && (cursor.prev().kind() == CharKind::PunctuationHalf
            && !cursor.prev().is_bracket()
            && !cursor.prev().is_quote()
            && cursor.prev() != Token::Char('\'')
            || cursor.next().kind() == CharKind::PunctuationHalf
                && !cursor.next().is_bracket()
                && !cursor.next().is_quote()
                && cursor.next() != Token::Char('\''))
    {
        return;
    }

    if config
        .rules
        .half_width_punctuation
        .contains(cursor.current().to_half_width())
    {
        cursor.replace(cursor.current().to_half_width());
    }

    if config
        .rules
        .full_width_punctuation
        .contains(cursor.current().to_full_width())
    {
        cursor.replace(cursor.current().to_full_width());
    }

    if cursor.current() == '"' && config.rules.full_width_punctuation.contains("“”") {
        if ctx.half_width_double_quote_count % 2 == 1 {
            cursor.replace('“');
        } else {
            cursor.replace('”');
        }
    }
    if cursor.current() == '\'' && config.rules.full_width_punctuation.contains("‘’") {
        if ctx.half_width_single_quote_count % 2 == 1 {
            cursor.replace('‘');
        } else {
            cursor.replace('’');
        }
    }
}
