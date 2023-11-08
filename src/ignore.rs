use std::ops::Range;

use pulldown_cmark::{Event, Parser};
use regex::{Error, Regex};

const IGNORE_CAPTURE_NAME: &str = "ignore";
const DISABLE_HTML_RE: &str = r"^\s*<!--\s*zhlint disabled\s*-->\s*$";
const IGNORE_HTML_RE: &str = r"^\s*<!--\s*zhlint ignore:(?<regex>.*)-->\s*$";

#[derive(Debug, Clone)]
pub enum Ignore {
    Disabled,
    Ignore(Vec<String>),
}

pub(crate) fn get_ignore_list_from_events(events: Parser) -> Ignore {
    let mut res = Vec::new();
    let disable_re = Regex::new(DISABLE_HTML_RE).unwrap();
    let ignore_re = Regex::new(IGNORE_HTML_RE).unwrap();

    for event in events {
        if let Event::Html(s) = event {
            if disable_re.is_match(&s) {
                return Ignore::Disabled;
            }
            if let Some(re) = ignore_re
                .captures(&s)
                .and_then(|x| x.name("regex"))
                .map(|x| x.as_str())
            {
                res.push(re.trim().to_string());
            }
        }
    }
    Ignore::Ignore(res)
}

pub(crate) fn get_ignore_ranges<T: AsRef<str>>(
    text: &str,
    ignores: &[T],
) -> Result<Vec<Range<usize>>, Error> {
    let mut res = Vec::new();
    for ignore_regex in ignores {
        let re = Regex::new(ignore_regex.as_ref())?;
        if re
            .capture_names()
            .any(|x| x.is_some_and(|x| x == IGNORE_CAPTURE_NAME))
        {
            res.extend(
                re.captures_iter(text)
                    .map(|x| x.name(IGNORE_CAPTURE_NAME).map(|x| x.range()).unwrap()),
            );
        } else {
            res.extend(
                Regex::new(ignore_regex.as_ref())?
                    .find_iter(text)
                    .map(|x| x.range()),
            );
        }
    }
    Ok(res)
}
