//! This rule is used to revert changes of spaceAfter between numbers and
//! Chinese units.

use crate::{char_kind::CharKindTrait, config::Config, cursor::Cursor, nodes::Node};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // make sure the value is a number
    if let Node::HalfwidthContent { value, .. } = cursor.current() {
        if !value.chars().all(|c| c.is_numeric() && !c.is_wide()) {
            return;
        }
    } else {
        return;
    }

    // make sure the value after is a Chinese unit
    if let Some(Node::FullwidthContent { value, .. }) = cursor.after_visible() {
        if !config.skip_zh_units.contains(value) {
            return;
        }
    } else {
        return;
    }

    if let Some(before) = cursor.before_visible_mut() {
        // TODO: make sure there is no space between originally
        // if !before.space_after().is_empty() {
        //     return;
        // }
        // revert non-space before
        before.revert_space_after();
    }
    // revert non-space after
    cursor.current_mut().revert_space_after();
}
