//! This rule is used to revert changes of abbreviations.
//!
//! Details:
//! - the point is rever the trailing dot

use crate::{config::Config, cursor::Cursor, nodes::Node};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // skip non-dot tokens
    if !cursor.current().is_char_and(|c| c == &'.') {
        return;
    }

    // make sure it's the ending dot of the abbr
    if let Some(after) = cursor.after() {
        if matches!(after, Node::HalfwidthContent { .. }) && !cursor.current().has_space_after() {
            return;
        }
    }

    // keep the dot if the previous tokens match any abbr
    if cursor.match_abbr(&config.skip_abbrs) {
        if let Node::Char {
            value,
            space_after: _,
        } = &mut cursor.current_mut()
        {
            value.revert_modified()
        }
    }
}
