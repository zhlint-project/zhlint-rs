//! This rule will decide whether to keep a space outside inline code with
//! content like:
//! - xxx `foo` xxx
//! - xxx <code>foo</code> xxx
//! in markdown/html.
//!
//! Options:
//! - space_outside_code: Option<bool>
//!   - `true`: keep one space outside (default)
//!   - `false`: no space outside
//!   - `None`: do nothing, just keep the original format
//!
//! Details:
//! - code x code
//! - content x code
//! - code x content

use pulldown_cmark::Event;

use crate::{
    config::Config,
    cursor::Cursor,
    nodes::{Node, Space},
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    // skip if there is no options
    let space = match config.space_outside_code {
        Some(true) => Space::One,
        Some(false) => Space::Empty,
        None => return,
    };
    // skip non-code tokens
    if !matches!(
        cursor.current(),
        Node::Event {
            value: Event::Code(_),
            offset: _,
            space_after: _,
        }
    ) {
        return;
    }
    // skip non-after-token situations
    // content x code
    if let Some(before) = cursor.before_visible_mut() {
        before.modify_space_after(space.clone());
    }
    // code x content or code x code
    if let Some(after) = cursor.after_visible() {
        if matches!(
            after,
            Node::HalfwidthContent { .. }
                | Node::FullwidthContent { .. }
                | Node::Event {
                    value: Event::Code(_),
                    offset: _,
                    space_after: _,
                }
        ) {
            cursor.current_mut().modify_space_after(space.clone());
        }
    }
}
