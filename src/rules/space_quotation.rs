//! This rule is checking spaces besides quotations.
//!
//! Options
//! - no_space_inside_quotation: Option<bool> | undefined
//! - space_outside_halfwidth_quotation: boolean | undefined
//! - no_space_outside_fullwidth_quotation: boolean | undefined
//!
//! Details:
//! - no_space_inside_quotation:
//!   - left-quotation x right-quotation
//!   - content/punctuation/right-quotation/right-bracket/code/unknown/container x right-quotation
//!   - left-quotation x content/punctuation/left-quotation/left-bracket/code/unknown/container
//! - space_outside_halfwidth_quotation:
//!   - right-half-quotation x left-half-quotation
//!   - content/code x left-half-quotation
//!   - right-half-quotation x content/code
//! - no_space_outside_fullwidth_quotation:
//!   - right-full-quotation x left-full-quotation
//!   - content/code x left-full-quotation
//!   - right-full-quotation x content/code

use crate::{
    char_kind::CharKindTrait,
    config::Config,
    cursor::Cursor,
    nodes::{Node, Space},
};

pub fn rule(cursor: &mut Cursor, config: &Config) {
    let current_start;
    let current_end;
    if let Node::Group {
        start,
        inner_space_before,
        nodes,
        end,
        space_after: _,
    } = &mut cursor.current_mut()
    {
        current_start = start.clone();
        current_end = end.clone();

        // 1. no space inside quotation
        if config.no_space_inside_quotation {
            // 1.1 left-quotation x content/punctuation/left-quotation/left-bracket/code/unknown/container
            if nodes
                .first()
                .is_some_and(|x| !x.is_char_and(|c| c.is_right_punctuation()))
            {
                inner_space_before.to_be(Space::Empty);
            }

            // 1.2 content/punctuation/right-quotation/right-bracket/code/unknown/container x right-quotation
            if let Some(last_inside_node) = nodes.last_mut() {
                if !last_inside_node.is_char_and(|c| c.is_left_punctuation()) {
                    last_inside_node.remove_space_after();
                }
            }

            // 1.3 left-quotation x right-quotation
            if nodes.is_empty() {
                inner_space_before.to_be(Space::Empty);
            }
        }
    } else {
        // skip non-group tokens
        return;
    }

    // 2. space outside half/full quotation
    if !config.no_space_outside_fullwidth_quotation
        && config.space_outside_halfwidth_quotation.is_none()
    {
        return;
    }

    // 2.1 right-quotation x left-quotation
    if let Some(Node::Group { start, .. }) = cursor.after_visible() {
        // 2.1.1 right-full-quotation x left-full-quotation
        // 2.1.2 right-half-quotation x left-half-quotation
        if current_end.modified().is_wide_or_chinese() || start.modified().is_wide_or_chinese() {
            if config.no_space_outside_fullwidth_quotation {
                cursor.current_mut().remove_space_after();
            }
        } else if let Some(space) = config.space_outside_halfwidth_quotation {
            cursor.current_mut().modify_space_after(space.into())
        }
    }

    // 2.2 content/code x left-quotation
    if let Some(before) = cursor.before_visible_mut() {
        // 2.2.1 content/code x left-full-quotation
        // 2.2.2 content/code x left-half-quotation
        if current_start.modified().is_wide_or_chinese() {
            if config.no_space_outside_fullwidth_quotation {
                before.remove_space_after();
            }
        } else if let Some(space) = config.space_outside_halfwidth_quotation {
            before.modify_space_after(space.into())
        }
    }

    // 2.3 right-quotation x content/code
    if cursor.after_visible().is_some() {
        // 2.3.1 right-full-quotation x content/code
        // 2.3.2 right-half-quotation x content/code
        if current_end.modified().is_wide_or_chinese() {
            if config.no_space_outside_fullwidth_quotation {
                cursor.current_mut().remove_space_after();
            }
        } else if let Some(space) = config.space_outside_halfwidth_quotation {
            cursor.current_mut().modify_space_after(space.into())
        }
    }
}
