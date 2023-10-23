use zh_formatter::{
    config::{Config, ZhScript},
    run_text,
};

#[test]
fn test_trim_space() {
    let config = Config {
        trim_space: true,
        ..Default::default()
    };
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
fn test_punctuation_width() {
    let config = Config {
        half_width_punctuation: "()".to_string(),
        full_width_punctuation: "，。：；？！“”‘’".to_string(),
        ..Default::default()
    };
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
        run_text(r#"foo bar   baz"#, &Config::default()),
        r#"foo bar baz"#
    );
    assert_eq!(
        run_text(
            r#"中文 中文 中 文"#,
            &Config {
                no_space_between_full_width_letters: true,
                ..Default::default()
            }
        ),
        r#"中文中文中文"#
    );
    assert_eq!(
        run_text(
            r#"中文foo 中文 foo中foo文"#,
            &Config {
                space_between_mixed_width_letters: Some(true),
                ..Default::default()
            }
        ),
        r#"中文 foo 中文 foo 中 foo 文"#
    );
    assert_eq!(
        run_text(
            r#"中文foo 中文 foo中foo文"#,
            &Config {
                space_between_mixed_width_letters: Some(false),
                ..Default::default()
            }
        ),
        r#"中文foo中文foo中foo文"#
    );
}

#[test]
fn test_space_punctuation() {
    let config = Config {
        no_space_before_punctuation: true,
        ..Default::default()
    };
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

    let config = Config {
        space_after_half_width_punctuation: Some(true),
        ..Default::default()
    };
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

    let config = Config {
        no_space_after_full_width_punctuation: true,
        ..Default::default()
    };
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
    let config = Config {
        no_space_inside_quote: true,
        ..Default::default()
    };
    assert_eq!(run_text(r#"foo " bar " baz"#, &config), r#"foo "bar" baz"#);
    assert_eq!(run_text(r#"foo “ bar ” baz"#, &config), r#"foo “bar” baz"#);

    let config = Config {
        space_outside_half_quote: Some(true),
        ..Default::default()
    };
    assert_eq!(
        run_text(r#"foo " bar " baz"#, &config),
        r#"foo " bar " baz"#
    );
    assert_eq!(
        run_text(r#"foo “ bar ” baz"#, &config),
        r#"foo “ bar ” baz"#
    );
    assert_eq!(run_text(r#"一 " 二 " 三"#, &config), r#"一 " 二 " 三"#);
    let config = Config {
        space_outside_half_quote: Some(false),
        ..Default::default()
    };
    assert_eq!(run_text(r#"foo " bar " baz"#, &config), r#"foo" bar "baz"#);
    assert_eq!(run_text(r#"一 " 二 " 三"#, &config), r#"一" 二 "三"#);
    assert_eq!(run_text(r#"一 “ 二 ” 三"#, &config), r#"一 “ 二 ” 三"#);

    let config = Config {
        no_space_outside_full_quote: true,
        ..Default::default()
    };
    assert_eq!(run_text(r#"一 “ 二 ” 三"#, &config), r#"一“ 二 ”三"#);
    assert_eq!(run_text(r#"foo “ bar ” baz"#, &config), r#"foo“ bar ”baz"#);
    assert_eq!(run_text(r#"一 “ 二 ” 三"#, &config), r#"一“ 二 ”三"#);
}

#[test]
fn test_space_bracket() {
    let config = Config {
        no_space_inside_bracket: true,
        ..Default::default()
    };
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

    let config = Config {
        space_outside_half_bracket: Some(true),
        ..Default::default()
    };
    assert_eq!(
        run_text(r#"foo ( bar ) baz"#, &config),
        r#"foo ( bar ) baz"#
    );
    // TODO: skip content x bracket x content without space
    // assert_eq!(run_text(r#"foo(bar)baz"#, &config), r#"foo(bar)baz"#);
    let config = Config {
        space_outside_half_bracket: Some(false),
        ..Default::default()
    };
    assert_eq!(run_text(r#"foo(bar)baz"#, &config), r#"foo(bar)baz"#);
    assert_eq!(run_text(r#"foo ( bar ) baz"#, &config), r#"foo( bar )baz"#);

    let config = Config {
        no_space_outside_full_bracket: true,
        ..Default::default()
    };
    assert_eq!(run_text(r#"foo（bar）baz"#, &config), r#"foo（bar）baz"#);
    assert_eq!(
        run_text(r#"foo （ bar ） baz"#, &config),
        r#"foo（ bar ）baz"#
    );
}
