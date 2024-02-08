use zhlint::{
    config::{Config, ZhScript},
    run,
};

fn run_text(text: &str, config: &Config) -> String {
    run(text, config).text
}

#[test]
#[ignore]
fn test_trim_space() {
    let mut config = Config::empty();
    config.trim_space = true;

    assert_eq!(run_text(r#""#, &config), r#""#);
    assert_eq!(run_text(r#" `foo` "foo" "#, &config), r#"`foo` "foo""#);
    assert_eq!(run_text(r#" foo bar   "#, &config), r#"foo bar"#);
    assert_eq!(run_text(r#"中文, 中文. "#, &config), r#"中文, 中文."#);
    assert_eq!(
        run_text(r#"中文, 中文.中； 文。 "#, &config),
        r#"中文, 中文.中； 文。"#
    );
    assert_eq!(run_text(r#" " bar " "#, &config), r#"" bar ""#);
    assert_eq!(run_text(r#" (bar) "#, &config), r#"(bar)"#);
}

#[test]
#[ignore]
fn test_hyper_mark() {
    let mut config = Config::empty();
    config.no_space_inside_hyper_mark = true;

    assert_eq!(run_text(r#"x [ yyy ]() z"#, &config), r#"x [yyy]() z"#);
    // assert_eq!(run_text(r#"x _** yyy ** _ z"#, &config), r#"x _**yyy**_ z"#);
    // assert_eq!(run_text(r#"x _ ** yyy **_ z"#, &config), r#"x _**yyy**_ z"#);
    // assert_eq!(run_text(r#"_ ** yyy **_"#, &config), r#"_**yyy**_"#);
}

#[test]
fn test_hyper_code() {
    let mut config = Config::empty();
    config.space_outside_code = Some(true);

    assert_eq!(run_text(r#"xxx`foo`xxx"#, &config), r#"xxx `foo` xxx"#);
    assert_eq!(run_text(r#"xxx`foo` xxx"#, &config), r#"xxx `foo` xxx"#);
    assert_eq!(run_text(r#"xxx `foo`xxx"#, &config), r#"xxx `foo` xxx"#);
    assert_eq!(run_text(r#"xxx `foo` xxx"#, &config), r#"xxx `foo` xxx"#);
    assert_eq!(run_text(r#"xxx ` foo`xxx"#, &config), r#"xxx ` foo` xxx"#);
}

#[test]
fn test_punctuation_width() {
    let mut config = Config::empty();
    config.halfwidth_punctuation = "()".to_string();
    config.fullwidth_punctuation = "，。：；？！“”‘’".to_string();

    assert_eq!(run_text(r#"你好,再见."#, &config), r#"你好，再见。"#);
    assert_eq!(run_text(r#"你（好）,再见."#, &config), r#"你(好)，再见。"#);
    assert_eq!(run_text(r#"你'好',再见."#, &config), r#"你‘好’，再见。"#);
    assert_eq!(run_text(r#"你"好",再见."#, &config), r#"你“好”，再见。"#);
    assert_eq!(
        run_text(r#""你'好'",再见."#, &config),
        r#"“你‘好’”，再见。"#
    );
    assert_eq!(run_text(r#"what's up"#, &config), r#"what's up"#);
}

#[test]
fn test_punctuation_unification() {
    assert_eq!(
        run_text(
            r#"老師說：「你們要記住國父說的『青年要立志做大事，不要做大官』這句話。」"#,
            &Config {
                unified_punctuation: Some(ZhScript::Simplified),
                ..Default::default()
            }
        ),
        r#"老師說：“你們要記住國父說的‘青年要立志做大事，不要做大官’這句話。”"#
    );
    assert_eq!(
        run_text(
            r#"老師說：“你們要記住國父說的‘青年要立志做大事，不要做大官’這句話。”"#,
            &Config {
                unified_punctuation: Some(ZhScript::Traditional),
                ..Default::default()
            }
        ),
        r#"老師說：「你們要記住國父說的『青年要立志做大事，不要做大官』這句話。」"#
    );
}

#[test]
fn test_space_letters() {
    assert_eq!(
        run_text(r#"foo bar   baz"#, &Config::empty()),
        r#"foo bar   baz"#
    ); // diff
    assert_eq!(
        run_text(
            r#"中文 中文 中 文"#,
            &Config {
                no_space_between_fullwidth_content: true,
                ..Default::default()
            }
        ),
        r#"中文中文中文"#
    );
    assert_eq!(
        run_text(
            r#"中文foo 中文 foo中foo文"#,
            &Config {
                space_between_mixedwidth_letters: Some(true),
                ..Default::default()
            }
        ),
        r#"中文 foo 中文 foo 中 foo 文"#
    );
    assert_eq!(
        run_text(
            r#"中文foo 中文 foo中foo文"#,
            &Config {
                space_between_mixedwidth_letters: Some(false),
                ..Default::default()
            }
        ),
        r#"中文foo中文foo中foo文"#
    );
}

#[test]
fn test_space_punctuation() {
    let mut config = Config::empty();
    config.no_space_before_pause_or_stop = true;

    assert_eq!(
        run_text(r#"中文 , 一. 二 ；三。四"#, &config),
        r#"中文, 一. 二；三。四"#
    );
    assert_eq!(
        run_text(r#"foo, " bar " , baz"#, &config),
        r#"foo, " bar ", baz"#
    );
    assert_eq!(
        run_text(r#"foo. “ bar ” . baz"#, &config),
        r#"foo. “ bar ”. baz"#
    );
    assert_eq!(
        run_text(r#"一， " 二 " ， 三"#, &config),
        r#"一， " 二 "， 三"#
    );
    assert_eq!(
        run_text(r#"一。 “ 二 ” 。 三"#, &config),
        r#"一。 “ 二 ”。 三"#
    );
    assert_eq!(
        run_text(r#"foo, " bar " , baz"#, &config),
        r#"foo, " bar ", baz"#
    );
    assert_eq!(
        run_text(r#"foo. “ bar ” . baz"#, &config),
        r#"foo. “ bar ”. baz"#
    );
    assert_eq!(
        run_text(r#"一， " 二 " ， 三"#, &config),
        r#"一， " 二 "， 三"#
    );
    assert_eq!(
        run_text(r#"一。 “ 二 ” 。 三"#, &config),
        r#"一。 “ 二 ”。 三"#
    );

    let mut config = Config::empty();
    config.space_after_halfwidth_pause_or_stop = Some(true);

    assert_eq!(
        run_text(r#"中文, 中文.中； 文。中文"#, &config),
        r#"中文, 中文. 中； 文。中文"#
    );
    assert_eq!(
        run_text(r#"foo," bar " , baz"#, &config),
        r#"foo, " bar " , baz"#
    );
    assert_eq!(
        run_text(r#"foo.“ bar ” . baz"#, &config),
        r#"foo. “ bar ” . baz"#
    );

    let mut config = Config::empty();
    config.no_space_after_fullwidth_pause_or_stop = true;

    assert_eq!(
        run_text(r#"中文, 中文.中； 文。中文"#, &config),
        r#"中文, 中文.中；文。中文"#
    );
    assert_eq!(
        run_text(r#"一， " 二 " ， 三"#, &config),
        r#"一，" 二 " ，三"#
    );
    assert_eq!(
        run_text(r#"一。 “ 二 ” 。 三"#, &config),
        r#"一。“ 二 ” 。三"#
    );
}

#[test]
fn test_space_quote() {
    let mut config = Config::empty();
    config.no_space_inside_quotation = true;

    assert_eq!(run_text(r#"foo " bar " baz"#, &config), r#"foo "bar" baz"#);
    assert_eq!(run_text(r#"foo “ bar ” baz"#, &config), r#"foo “bar” baz"#);

    let mut config = Config::empty();
    config.space_outside_halfwidth_quotation = Some(true);

    assert_eq!(
        run_text(r#"foo " bar " baz"#, &config),
        r#"foo " bar " baz"#
    );
    assert_eq!(
        run_text(r#"foo “ bar ” baz"#, &config),
        r#"foo “ bar ” baz"#
    );
    assert_eq!(run_text(r#"一 " 二 " 三"#, &config), r#"一 " 二 " 三"#);

    let mut config = Config::empty();
    config.space_outside_halfwidth_quotation = Some(false);

    assert_eq!(run_text(r#"foo " bar " baz"#, &config), r#"foo" bar "baz"#);
    assert_eq!(run_text(r#"一 " 二 " 三"#, &config), r#"一" 二 "三"#);
    assert_eq!(run_text(r#"一 “ 二 ” 三"#, &config), r#"一 “ 二 ” 三"#);

    let mut config = Config::empty();
    config.no_space_outside_fullwidth_quotation = true;

    assert_eq!(run_text(r#"一 “ 二 ” 三"#, &config), r#"一“ 二 ”三"#);
    assert_eq!(run_text(r#"foo “ bar ” baz"#, &config), r#"foo“ bar ”baz"#);
    assert_eq!(run_text(r#"一 “ 二 ” 三"#, &config), r#"一“ 二 ”三"#);
}

#[test]
fn test_space_bracket() {
    let mut config = Config::empty();
    config.no_space_inside_bracket = true;

    assert_eq!(run_text(r#"foo (bar) baz"#, &config), r#"foo (bar) baz"#);
    assert_eq!(run_text(r#"foo ( bar ) baz"#, &config), r#"foo (bar) baz"#);
    assert_eq!(
        run_text(r#"foo （bar） baz"#, &config),
        r#"foo （bar） baz"#
    );
    assert_eq!(
        run_text(r#"foo （ bar ） baz"#, &config),
        r#"foo （bar） baz"#
    );

    let mut config = Config::empty();
    config.space_outside_halfwidth_bracket = Some(true);

    assert_eq!(
        run_text(r#"foo ( bar ) baz"#, &config),
        r#"foo ( bar ) baz"#
    );
    assert_eq!(run_text(r#"foo(bar)baz"#, &config), r#"foo(bar)baz"#);

    let mut config = Config::empty();
    config.space_outside_halfwidth_bracket = Some(false);

    assert_eq!(run_text(r#"foo(bar)baz"#, &config), r#"foo(bar)baz"#);
    assert_eq!(run_text(r#"foo ( bar ) baz"#, &config), r#"foo( bar )baz"#);

    let mut config = Config::empty();
    config.no_space_outside_fullwidth_bracket = true;
    assert_eq!(run_text(r#"foo（bar）baz"#, &config), r#"foo（bar）baz"#);
    assert_eq!(
        run_text(r#"foo （ bar ） baz"#, &config),
        r#"foo（ bar ）baz"#
    );
}
