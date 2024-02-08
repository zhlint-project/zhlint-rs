//! This rule will format each punctuation into the right width options.
//!
//! Options:
//! - half_width_punctuation: string = `()`
//! - full_width_punctuation: string = `，。：；？！“”‘’`
//!
//! Details:
//! - skip half-width punctuations between half-width content without space
//! - skip successive multiple half-width punctuations

use crate::{
    char_kind::CharKindTrait,
    config::Config,
    cursor::Cursor,
    nodes::{Node, OffsetValue},
};

fn to_halfwidth(c: char) -> char {
    match c {
        '，' => ',',
        '。' => '.',
        '；' => ';',
        '：' => ':',
        '？' => '?',
        '！' => '!',
        '（' => '(',
        '）' => ')',
        '［' => '[',
        '］' => ']',
        '｛' => '{',
        '｝' => '}',
        '“' | '”' => '"',
        '‘' | '’' => '\'',
        _ => c,
    }
}

fn to_fullwidth(c: char) -> char {
    match c {
        ',' => '，',
        '.' => '。',
        ';' => '；',
        ':' => '：',
        '?' => '？',
        '!' => '！',
        '(' => '（',
        ')' => '）',
        '[' => '［',
        ']' => '］',
        '{' => '｛',
        '}' => '｝',
        _ => c,
    }
}

pub fn rule(cursor: &mut Cursor, config: &Config) {
    let check_char_offset = |value: &mut OffsetValue<char>| {
        if config.halfwidth_punctuation.contains(to_halfwidth(**value)) {
            value.to_be(to_halfwidth(**value));
        }
        if config.fullwidth_punctuation.contains(to_fullwidth(**value)) {
            value.to_be(to_fullwidth(**value));
        }
    };

    // 1. quotations in the alter pair map
    if let Node::Group {
        start,
        inner_space_before: _,
        nodes: _,
        end,
        space_after: _,
    } = &mut cursor.current_mut()
    {
        if start.modified() == &'"' && config.fullwidth_punctuation.contains('“') {
            start.to_be('“');
        } else if start.modified() == &'\'' && config.fullwidth_punctuation.contains('‘') {
            start.to_be('‘');
        } else {
            check_char_offset(start);
        }

        if end.modified() == &'"' && config.fullwidth_punctuation.contains('”') {
            end.to_be('”');
        } else if end.modified() == &'\'' && config.fullwidth_punctuation.contains('’') {
            end.to_be('’');
        } else {
            check_char_offset(end);
        }
        return;
    }

    // skip non-punctuation/quotation/bracket situations
    if !cursor.current().is_char_and(|c| c.is_punctuation()) {
        return;
    }
    // skip halfwidth punctuations between halfwidth content without space
    if cursor.is_halfwidth_punctuation_without_space_around() {
        return;
    }
    // skip successive multiple half-width punctuations
    if cursor.is_successive_halfwidth_punctuation() {
        return;
    }

    // 1. normal punctuations in the alter width map
    // 2. brackets in the alter width map
    if let Node::Char {
        value,
        space_after: _,
    } = &mut cursor.current_mut()
    {
        if value.modified().is_single_punctuation() || value.modified().is_bracket_punctuation() {
            check_char_offset(value);
        }
    }
}
