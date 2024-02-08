use crate::{
    config::Config,
    cursor::Cursor,
    errors::ZhlintError,
    lexer::Lexer,
    nodes::{Node, OffsetValue, Space},
    parser::Parser,
    rules::rules,
};

pub mod char_kind;
pub mod config;
pub mod cursor;
pub mod errors;
pub mod ignore;
pub mod lexer;
pub mod nodes;
pub mod parser;
pub mod rules;

/*
pub fn run<W: fmt::Write>(text: &str, config: &Config, mut writer: W) -> Result<(), fmt::Error> {
    let (front_matter, text) = cut_front_matter(text);

    let rules = rules();

    let options = Options::empty();

    let mut ignore = match get_ignore_list_from_events(Parser::new_ext(text, options)) {
        Ignore::Disabled => {
            writer.write_str(front_matter)?;
            writer.write_str(text)?;
            return Ok(());
        }
        Ignore::Ignore(ignore) => ignore,
    };
    ignore.append(&mut config.ignores.clone());

    let ignore_ranges = get_ignore_ranges(text, &ignore).unwrap();

    let mut event_cursor = EventCursor::new(Parser::new_ext(text, options).into_offset_iter());
    let mut context = Context::new();

    let t = iter::from_fn(|| {
        if let Some(event) = &event_cursor.current_event {
            let mut res = event.0.clone();
            match &event.0 {
                Event::Start(tag) => {
                    if !matches!(
                        tag,
                        Tag::Emphasis
                            | Tag::Strong
                            | Tag::Strikethrough
                            | Tag::Link(..)
                            | Tag::Image(..)
                    ) {
                        context.clear();
                    }
                    if matches!(tag, Tag::CodeBlock(_)) {
                        context.code_block_mark = true
                    }
                }
                Event::End(Tag::CodeBlock(_)) => context.code_block_mark = false,
                Event::Text(_) if !context.code_block_mark => {
                    let mut text_cursor = event_cursor.to_text_cursor().unwrap();

                    loop {
                        if text_cursor.current() == '\'' {
                            context.half_width_single_quote_count += 1;
                        }
                        if text_cursor.current() == '"' {
                            context.half_width_double_quote_count += 1;
                        }

                        text_cursor.skip_str(&config.skip_abbrs);

                        let mut skip_flag = false;

                        for ignore_range in &ignore_ranges {
                            let (_, current_offset, _) = text_cursor.current_offset();
                            if ignore_range.contains(&current_offset) {
                                skip_flag = true;
                                break;
                            }
                        }

                        if !skip_flag {
                            for rule in &rules {
                                rule(&mut context, &mut text_cursor, config);
                            }
                        }

                        if !text_cursor.advance() {
                            break;
                        }
                    }

                    res = Event::Text(String::from(text_cursor).into());
                }
                _ => (),
            };
            event_cursor.advance();
            Some(res)
        } else {
            None
        }
    });

    writer.write_str(front_matter)?;
    cmark(t, writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let text = r#"---
a = 1
b = 2
---
# 1
## 2
"#;

        let mut res = String::new();

        run(text, &Config::default(), &mut res).unwrap();

        println!("{}", res);
    }
}
*/

#[derive(Debug, Clone)]
pub struct Report {
    pub text: String,
    pub errors: Vec<ZhlintError>,
}

impl Report {
    fn report_nodes(&mut self, nodes: &[Node<'_>]) {
        for node in nodes.iter().rev() {
            match &node {
                Node::Char { value, space_after } => {
                    self.report_space(space_after);
                    self.report_char(value);
                }
                Node::HalfwidthContent { space_after, .. } => self.report_space(space_after),
                Node::FullwidthContent { space_after, .. } => self.report_space(space_after),
                Node::Event {
                    value: _,
                    offset: _,
                    space_after,
                } => self.report_space(space_after),
                Node::Group {
                    start,
                    inner_space_before,
                    nodes,
                    end,
                    space_after,
                } => {
                    self.report_space(space_after);
                    self.report_char(end);
                    self.report_nodes(nodes);
                    self.report_space(inner_space_before);
                    self.report_char(start);
                }
            }
        }
    }

    fn report_char(&mut self, value: &OffsetValue<char>) {
        if let Some(modified) = value.modified {
            self.text
                .replace_range(value.offset.clone(), &modified.to_string());
            self.errors.push(ZhlintError::CharError {
                value: value.original,
                modified,
                offset: value.offset.clone(),
            });
        }
    }

    fn report_space(&mut self, value: &OffsetValue<Space>) {
        if let Some(modified) = &value.modified {
            self.text
                .replace_range(value.offset.clone(), &modified.to_string());
            self.errors.push(ZhlintError::SpaceError {
                value: value.original.clone(),
                modified: modified.clone(),
                offset: value.offset.clone(),
            });
        }
    }
}

pub fn run(text: &str, config: &Config) -> Report {
    let mut errors = Vec::new();

    let mut parser = Parser::new(Lexer::new(text));
    let mut paragraphs = parser.parse();

    for paragraph_nodes in &mut paragraphs {
        match paragraph_nodes {
            Ok(paragraph_nodes) => run_tokens(&mut paragraph_nodes.0, config),
            Err(e) => errors.push(e),
        }
    }

    // println!("{:#?}", paragraphs);
    // println!("{:#?}", text.char_indices().collect::<Vec<_>>());

    let mut report = Report {
        text: text.to_owned(),
        errors: Vec::new(),
    };

    // if config.trim_space {
    //     let mut start_space = String::new();
    //     let mut start_offset = 0..0;
    //     let mut end_space = String::new();
    //     let mut end_offset = report.text.len()..report.text.len();
    //     let mut chars = report.text.char_indices();
    //     for (i, c) in chars.by_ref() {
    //         if !c.is_space() {
    //             break;
    //         }
    //         start_space.push(c);
    //         start_offset.end = i + c.len_utf8();
    //     }
    //     while let Some((i, c)) = chars.next_back() {
    //         if !c.is_space() {
    //             break;
    //         }
    //         end_space.push(c);
    //         end_offset.start = i;
    //     }

    //     if !end_space.is_empty() {
    //         report.report_space(&OffsetValue {
    //             original: end_space.into(),
    //             modified: Some(Space::Empty),
    //             offset: end_offset,
    //         });
    //     }
    //     if !start_space.is_empty() {
    //         report.report_space(&OffsetValue {
    //             original: start_space.into(),
    //             modified: Some(Space::Empty),
    //             offset: start_offset,
    //         });
    //     }
    // }

    for paragraph_nodes in paragraphs.iter().flatten().rev() {
        report.report_nodes(&paragraph_nodes.0);
    }

    for error in report.errors.clone().into_iter().rev() {
        let error = miette::Error::new(error).with_source_code(text.to_string());
        println!("{:?}", error);
    }

    report
}

fn run_tokens(nodes: &mut Vec<Node<'_>>, config: &Config) {
    for i in 0..nodes.len() {
        if let Node::Group { nodes, .. } = &mut nodes[i] {
            run_tokens(nodes, config);
        }
        let mut cursor = Cursor::new(nodes, i);
        for rule in rules() {
            rule(&mut cursor, config);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let md = r#"A:" it *abc*" end a b c
老師說：「你們要記住國父說的『青年要立志做大事，不要做大官』這句話。」"#;

        let report = run(md, &Config::default());
        println!("{}", report.text);
    }
}
