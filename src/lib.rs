use std::{fmt, iter};

use pulldown_cmark::{Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;

use crate::{
    config::Config,
    ignore::{get_ignore_list_from_events, get_ignore_ranges, Ignore},
    parser::EventCursor,
    rules::rules,
};

pub mod char_kind;
pub mod config;
pub mod ignore;
pub mod parser;
pub mod rules;

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub code_block_mark: bool,

    pub half_width_single_quote_count: u32,
    pub half_width_double_quote_count: u32,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.half_width_single_quote_count = 0;
        self.half_width_double_quote_count = 0;
    }
}

const FRONT_MATTER_DELIMITERS: [&str; 2] = ["---\n", "+++\n"];

fn cut_front_matter(text: &str) -> (&str, &str) {
    for front_matter_delimiter in FRONT_MATTER_DELIMITERS {
        if let Some(slice) = text.strip_prefix(front_matter_delimiter) {
            if let Some(index_of_ending_line) = slice.find(front_matter_delimiter) {
                return text.split_at(index_of_ending_line + front_matter_delimiter.len() * 2);
            }
        }
    }

    ("", text)
}

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

                        text_cursor.skip_str(&config.rules.skip_abbrs);

                        let mut skip_flag = false;

                        for ignore_range in &ignore_ranges {
                            if let Some(current_offset) = text_cursor.current_offset() {
                                if ignore_range.contains(&current_offset) {
                                    skip_flag = true;
                                    break;
                                }
                            }
                        }

                        if !skip_flag {
                            for rule in &rules {
                                rule(&context, &mut text_cursor, config);
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
