use std::io::BufWriter;
use std::str;

use comrak::{
    format_commonmark,
    nodes::{AstNode, NodeValue},
    parse_document, Arena, Options,
};
use itertools::Itertools;
use regex::Regex;

use crate::{
    config::Config,
    parser::{tokenize, CharKind, CharKindTrait, Cursor},
};

pub mod config;
pub mod parser;
pub mod rules;

pub fn run_text(mut text: &str, config: &Config) -> String {
    if config.trim_space {
        text = text.trim();
    }
    let mut tokens = tokenize(text).collect::<Vec<_>>();
    if tokens.is_empty() {
        return "".to_string();
    }
    let mut cursor = Cursor::new(&mut tokens);
    loop {
        cursor.skip_str(&config.skip_abbrs);
        for rule in rules::rules() {
            rule(&mut cursor, config);
        }
        if let Some(c) = cursor.advance() {
            cursor = c;
        } else {
            break;
        }
    }
    tokens.into_iter().filter(|x| x != &'\0').collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkdownNodeKind {
    Text,
    Code,
    Link,
    Other,
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, config: &Config, f: &F)
where
    F: Fn(&'a AstNode<'a>, &Config),
{
    f(node, config);

    let mut previous_kind = MarkdownNodeKind::Other;
    let mut previous_node: Option<
        &comrak::arena_tree::Node<'_, std::cell::RefCell<comrak::nodes::Ast>>,
    > = None;

    for c in node.children() {
        iter_nodes(c, config, f);

        match &mut c.data.borrow_mut().value {
            NodeValue::Text(ref mut text) => {
                let first_char = text.chars().next().unwrap_or('\0');
                if previous_kind == MarkdownNodeKind::Code && !first_char.is_punctuation() {
                    match config.space_outside_code {
                        Some(true) => {
                            if first_char.kind() != CharKind::Space {
                                text.insert(0, ' ');
                            }
                        }
                        Some(false) => {
                            if first_char.kind() == CharKind::Space {
                                text.remove(0);
                            }
                        }
                        None => (),
                    }
                }
                if previous_kind == MarkdownNodeKind::Link && !first_char.is_punctuation() {
                    match config.space_outside_link {
                        Some(true) => {
                            if first_char.kind() != CharKind::Space {
                                text.insert(0, ' ');
                            }
                        }
                        Some(false) => {
                            if first_char.kind() == CharKind::Space {
                                text.remove(0);
                            }
                        }
                        None => (),
                    }
                }
                previous_kind = MarkdownNodeKind::Text;
            }
            NodeValue::Code(_) => {
                if previous_kind == MarkdownNodeKind::Text {
                    if let NodeValue::Text(ref mut text) =
                        previous_node.unwrap().data.borrow_mut().value
                    {
                        let last_char = text.chars().last().unwrap_or('\0');
                        if !last_char.is_punctuation() {
                            match config.space_outside_code {
                                Some(true) => {
                                    if last_char.kind() != CharKind::Space {
                                        text.push(' ');
                                    }
                                }
                                Some(false) => {
                                    if last_char.kind() == CharKind::Space {
                                        text.remove(text.len() - 1);
                                    }
                                }
                                None => (),
                            }
                        }
                    }
                }
                previous_kind = MarkdownNodeKind::Code;
            }
            NodeValue::Link(_) => {
                if previous_kind == MarkdownNodeKind::Text {
                    if let NodeValue::Text(ref mut text) =
                        previous_node.unwrap().data.borrow_mut().value
                    {
                        let last_char = text.chars().last().unwrap_or('\0');
                        if !last_char.is_punctuation() {
                            match config.space_outside_link {
                                Some(true) => {
                                    if last_char.kind() != CharKind::Space {
                                        text.push(' ');
                                    }
                                }
                                Some(false) => {
                                    if last_char.kind() == CharKind::Space {
                                        text.remove(text.len() - 1);
                                    }
                                }
                                None => (),
                            }
                        }
                    }
                }
                previous_kind = MarkdownNodeKind::Link;
            }
            _ => previous_kind = MarkdownNodeKind::Other,
        }
        previous_node = Some(c);
    }
}

pub fn run_markdown(markdown: &str, config: &Config) -> String {
    let mut replace_log: Vec<String> = Vec::new();

    let markdown = markdown
        .split('\n')
        .map(|x| {
            for ignore in &config.ignore {
                if Regex::new(ignore).unwrap().is_match(x) {
                    replace_log.push(x.to_string());
                    return format!("\n\n\u{FFFD}\u{FFFE}\u{FFFF}{}\n\n", replace_log.len() - 1);
                }
            }
            x.to_string()
        })
        .join("\n");

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.front_matter_delimiter = Some(config.front_matter_delimiter.to_owned());

    let arena = Arena::new();
    let root = parse_document(&arena, &markdown, &options);

    iter_nodes(root, config, &|node, config| {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            let orig: String = std::mem::take(text);
            *text = run_text(&orig, config);
        }
    });

    let mut buf = BufWriter::new(Vec::new());
    format_commonmark(root, &options, &mut buf).unwrap();
    let bytes = buf.into_inner().unwrap();
    let mut res = String::from_utf8(bytes).unwrap().trim().to_string() + "\n";

    for (i, log) in replace_log.iter().enumerate() {
        res = res.replacen(&format!("\u{FFFD}\u{FFFE}\u{FFFF}{}", i), log, 1);
    }

    res
}
