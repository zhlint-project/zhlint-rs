use zh_formatter::{config::Config, run_markdown};

pub fn run_text(text: &str, config: &Config) -> String {
    run_markdown(text, config)
        .trim_end_matches('\n')
        .to_string()
}

fn config() -> Config {
    toml::from_str("").unwrap()
}

#[test]
fn test_zh_units() {
    assert_eq!(
        run_text(
            r#"2019年06月26号 2019-06-26 12:00"#,
            &Config {
                space_after_half_width_punctuation: Some(true),
                ..Default::default()
            }
        ),
        r#"2019年06月26号 2019-06-26 12:00"#
    );
}

#[test]
fn test_abbrs() {
    assert_eq!(
        run_text(r#"运行时 + 编译器 vs. 只包含运行时"#, &config()),
        r#"运行时 + 编译器 vs. 只包含运行时"#
    );
}

#[test]
fn test_backslash() {
    assert_eq!(
        run_text(r"This\# is \#not a heading but a normal hash", &config()),
        r"This\# is \#not a heading but a normal hash"
    );
    assert_eq!(
        run_text(r"这个\#是普通的 \# 井号而不是标题", &config()),
        r"这个\#是普通的 \# 井号而不是标题"
    );
}

#[test]
fn test_ellipsis() {
    assert_eq!(run_text(r"aaa...bbb", &config()), r"aaa...bbb");
    assert_eq!(run_text(r"aaa... bbb", &config()), r"aaa... bbb");
    assert_eq!(run_text(r"aaa ...bbb", &config()), r"aaa...bbb");
    // assert_eq!(run_text(r"`aaa` ... `bbb`", &config()), r"`aaa` ... `bbb`");
}

#[test]
fn test_url() {
    assert_eq!(run_text(r"Vue.js 是什么", &config()), r"Vue.js 是什么");
    assert_eq!(run_text(r"www.vuejs.org", &config()), r"www.vuejs.org");
    assert_eq!(
        run_text(r"https://vuejs.org", &config()),
        r"https://vuejs.org"
    );
}

#[test]
fn test_slash_character() {
    assert_eq!(
        run_text(r"想知道 Vue 与其它库/框架有哪些区别", &config()),
        r"想知道 Vue 与其它库/框架有哪些区别"
    );
}

#[test]
fn test_special_character() {
    assert_eq!(
        run_text(r"Vue (读音 /vjuː/，类似于)", &config()),
        r"Vue (读音 /vjuː/，类似于)"
    );
}

#[test]
fn test_half_content_mark_half_content() {
    assert_eq!(run_text(r"a__[b](x)__c", &config()), r"a\_\_[b](x)\_\_c");
}

#[test]
fn test_plural_brackets() {
    assert_eq!(
        run_text(r"3 minite(s) left", &config()),
        r"3 minite(s) left"
    );
}

#[test]
fn test_single_quote_for_shorthand() {
    assert_eq!(
        run_text(r"how many user's here", &config()),
        r"how many user's here"
    );
    assert_eq!(
        run_text(r"how many users' items here", &config()),
        r"how many users' items here"
    );
    assert_eq!(run_text(r"what's going on", &config()), r"what's going on");
}

#[test]
fn test_math_exp() {
    assert_eq!(run_text(r"1+1=2", &config()), r"1+1=2");
    assert_eq!(run_text(r"a|b", &config()), r"a|b");
    assert_eq!(run_text(r"a | b", &config()), r"a | b");
    assert_eq!(run_text(r"a||b", &config()), r"a||b");
    assert_eq!(run_text(r"a || b", &config()), r"a || b");
}

#[test]
fn test_arrow_chars() {
    assert_eq!(
        run_text(r"Chrome 顶部导航 > 窗口 > 任务管理", &config()),
        r"Chrome 顶部导航 \> 窗口 \> 任务管理"
    );
}
