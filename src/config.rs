use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum ZhScript {
    #[default]
    Simplified,
    Traditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /* PRESET */
    /// Custom preset, currently only support:
    /// - `'default'`
    pub preset: Option<String>,

    /* PUNCTUATIONS */
    /// Convert these punctuations into half-width.
    /// default preset: `()`
    /// e.g. `（文字）` -> `(文字)`
    pub halfwidth_punctuation: String,

    /// Convert these punctuations into full-width.
    /// default preset: `，。：；？！“”‘’`
    /// e.g. `文字,文字.` -> `文字，文字。`
    pub fullwidth_punctuation: String,

    /// Convert traditional Chinese punctuations into simplified ones or vice versa.
    /// default preset: `simplified`
    /// e.g. `「文字」` -> `“文字”`
    pub unified_punctuation: Option<ZhScript>,

    /// Special case: skip `full_width_punctuation` for abbreviations.
    /// default preset:
    /// `['Mr.','Mrs.','Dr.','Jr.','Sr.','vs.','etc.','i.e.','e.g.','a.k.a']`
    pub skip_abbrs: Vec<String>,

    /* SPACES AROUND LETTERS */
    // default preset: `true`
    // - `true`: one space
    // - `false`: do nothing
    // e.g. `foo  bar` -> `foo bar`
    pub space_between_halfwidth_content: bool,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文 字` -> `文字`
    pub no_space_between_fullwidth_content: bool,

    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字 foo文字` -> `文字 foo 文字` (`true`)
    /// e.g. `文字foo 文字` -> `文字foo文字` (`false`)
    pub space_between_mixedwidth_letters: Option<bool>,

    /// Special case: skip `space_between_mixed_width_letters`
    /// for numbers x Chinese units.
    /// default preset: `['年','月','日','天','号','时','分','秒']`
    pub skip_zh_units: Vec<String>,

    /* SPACES AROUND PUNCTUATIONS */
    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字 ，文字` -> `文字，文字`
    pub no_space_before_pause_or_stop: bool,

    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字,文字` -> `文字, 文字` (`true`)
    /// e.g. `文字, 文字` -> `文字,文字` (`false`)
    pub space_after_halfwidth_pause_or_stop: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字， 文字` -> `文字，文字`
    pub no_space_after_fullwidth_pause_or_stop: bool,

    /* SPACES AROUND QUOTATIONS */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. `文字 "文字"文字` -> `文字 "文字" 文字` (`true`)
    /// e.g. `文字"文字" 文字` -> `文字"文字"文字` (`false`)
    pub space_outside_halfwidth_quotation: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字 “文字” 文字` -> `文字“文字”文字`
    pub no_space_outside_fullwidth_quotation: bool,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    /// e.g. `文字“ 文字 ”文字` -> `文字“文字”文字`
    pub no_space_inside_quotation: bool,

    /* SPACES AROUND BRACKETS */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    pub space_outside_halfwidth_bracket: Option<bool>,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    pub no_space_outside_fullwidth_bracket: bool,

    /// default preset: `true`
    /// - `true`: zero space
    /// - `false`: do nothing
    pub no_space_inside_bracket: bool,

    /* SPACES AROUND CODE */
    /// default preset: `true`
    /// - `true`: one space
    /// - `false`: zero space
    /// - `null`: do nothing
    /// e.g. '文字 `code`文字' -> '文字 `code` 文字' ('true')
    /// e.g. '文字`code` 文字' -> '文字`code`文字' ('false')
    pub space_outside_code: Option<bool>,

    /* SPACES AROUND MARKDOWN/HTML WRAPPERS */
    // default `true`
    // - `true`: zero space
    // - `undefined`: do nothing
    // e.g. `文字** foo **文字` -> `文字 **foo** 文字`
    pub no_space_inside_hyper_mark: bool,

    /* SPACES AT THE BEGINNING/END */
    /// default `true`
    /// e.g. ` 文字 ` -> `文字`
    pub trim_space: bool,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            preset: None,
            halfwidth_punctuation: String::new(),
            fullwidth_punctuation: String::new(),
            unified_punctuation: None,
            skip_abbrs: Vec::new(),
            space_between_halfwidth_content: false,
            no_space_between_fullwidth_content: false,
            space_between_mixedwidth_letters: None,
            skip_zh_units: Vec::new(),
            no_space_before_pause_or_stop: false,
            space_after_halfwidth_pause_or_stop: None,
            no_space_after_fullwidth_pause_or_stop: false,
            space_outside_halfwidth_quotation: None,
            no_space_outside_fullwidth_quotation: false,
            no_space_inside_quotation: false,
            space_outside_halfwidth_bracket: None,
            no_space_outside_fullwidth_bracket: false,
            no_space_inside_bracket: false,
            space_outside_code: None,
            no_space_inside_hyper_mark: false,
            trim_space: false,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            preset: None,
            halfwidth_punctuation: "()".to_string(),
            fullwidth_punctuation: "，。：；？！“”‘’".to_string(),
            unified_punctuation: Some(ZhScript::Simplified),
            skip_abbrs: vec![
                "Mr.".to_string(),
                "Mrs.".to_string(),
                "Dr.".to_string(),
                "Jr.".to_string(),
                "Sr.".to_string(),
                "vs.".to_string(),
                "etc.".to_string(),
                "i.e.".to_string(),
                "e.g.".to_string(),
                "a.k.a".to_string(),
            ],
            space_between_halfwidth_content: true,
            no_space_between_fullwidth_content: true,
            space_between_mixedwidth_letters: Some(true),
            skip_zh_units: vec![
                "年".to_string(),
                "月".to_string(),
                "日".to_string(),
                "天".to_string(),
                "号".to_string(),
                "时".to_string(),
                "分".to_string(),
                "秒".to_string(),
            ],
            no_space_before_pause_or_stop: true,
            space_after_halfwidth_pause_or_stop: Some(true),
            no_space_after_fullwidth_pause_or_stop: true,
            space_outside_halfwidth_quotation: Some(true),
            no_space_outside_fullwidth_quotation: true,
            no_space_inside_quotation: true,
            space_outside_halfwidth_bracket: Some(true),
            no_space_outside_fullwidth_bracket: true,
            no_space_inside_bracket: true,
            space_outside_code: Some(true),
            no_space_inside_hyper_mark: true,
            trim_space: true,
        }
    }
}
