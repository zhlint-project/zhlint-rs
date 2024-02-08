use zhlint::{config::Config, run};

fn run_text(text: &str, config: &Config) -> String {
    run(text, config).text
}

#[test]
fn test_zh_units() {
    assert_eq!(
        run_text(r#"2019年06月26号 2019-06-26 12:00"#, &Config::default()),
        r#"2019年06月26号 2019-06-26 12:00"#
    );
    assert_eq!(
        run_text(
            r#"1《测试》2【测试】3「测试」4（测试）"#,
            &Config::default()
        ),
        r#"1《测试》2【测试】3“测试”4(测试)"#
    ); // diff
    assert_eq!(run_text(r#"1？2！"#, &Config::default()), r#"1？2！"#);
}

#[test]
#[ignore]
fn test_abbrs() {
    assert_eq!(
        run_text(r#"运行时 + 编译器 vs. 只包含运行时"#, &Config::default()),
        r#"运行时 + 编译器 vs. 只包含运行时"#
    );
}

#[test]
fn test_backslash() {
    assert_eq!(
        run_text(
            r"This\# is \#not a heading but a normal hash",
            &Config::default()
        ),
        r"This\# is \#not a heading but a normal hash"
    );
    assert_eq!(
        run_text(r"这个\#是普通的 \# 井号而不是标题", &Config::default()),
        r"这个\#是普通的 \# 井号而不是标题"
    );
}

#[test]
fn test_ellipsis() {
    assert_eq!(run_text(r"aaa...bbb", &Config::default()), r"aaa...bbb");
    assert_eq!(run_text(r"aaa... bbb", &Config::default()), r"aaa... bbb");
    assert_eq!(run_text(r"aaa ...bbb", &Config::default()), r"aaa ...bbb");
    assert_eq!(
        run_text(r"`aaa` ... `bbb`", &Config::default()),
        r"`aaa` ... `bbb`"
    );
}

#[test]
fn test_url() {
    assert_eq!(
        run_text(r"Vue.js 是什么", &Config::default()),
        r"Vue.js 是什么"
    );
    assert_eq!(
        run_text(r"www.vuejs.org", &Config::default()),
        r"www.vuejs.org"
    );
    assert_eq!(
        run_text(r"https://vuejs.org", &Config::default()),
        r"https://vuejs.org"
    );
}

#[test]
fn test_slash_character() {
    assert_eq!(
        run_text(r"想知道 Vue 与其它库/框架有哪些区别", &Config::default()),
        r"想知道 Vue 与其它库/框架有哪些区别"
    );
}

#[test]
fn test_special_character() {
    assert_eq!(
        run_text(r"Vue (读音 /vjuː/，类似于)", &Config::default()),
        r"Vue (读音 /vjuː/，类似于)"
    );
}

#[test]
fn test_half_content_mark_half_content() {
    assert_eq!(
        run_text(r"a__[b](x)__c", &Config::default()),
        r"a__[b](x)__c"
    );
}

#[test]
fn test_plural_brackets() {
    assert_eq!(
        run_text(r"3 minite(s) left", &Config::default()),
        r"3 minite(s) left"
    );
}

#[test]
fn test_single_quote_for_shorthand() {
    assert_eq!(
        run_text(r"how many user's here", &Config::default()),
        r"how many user's here"
    );
    assert_eq!(
        run_text(r"how many users' items here", &Config::default()),
        r"how many users' items here"
    );
    assert_eq!(
        run_text(r"what's going on", &Config::default()),
        r"what's going on"
    );
}

#[test]
fn test_math_exp() {
    assert_eq!(run_text(r"1+1=2", &Config::default()), r"1+1=2");
    assert_eq!(run_text(r"a|b", &Config::default()), r"a|b");
    assert_eq!(run_text(r"a | b", &Config::default()), r"a | b");
    assert_eq!(run_text(r"a||b", &Config::default()), r"a||b");
    assert_eq!(run_text(r"a || b", &Config::default()), r"a || b");
}

#[test]
fn test_arrow_chars() {
    assert_eq!(
        run_text(r"Chrome 顶部导航 > 窗口 > 任务管理", &Config::default()),
        r"Chrome 顶部导航 > 窗口 > 任务管理"
    );
}
