use crate::Template;
use crate::tmpl;

#[test]
fn basic_param() {
    let mut tmpl = Template::from("Hello, <<<name>>>!".to_string());

    tmpl!(tmpl, name = "World");
    // tmpl.insert_param("name".to_string(), "World".to_string());

    assert_eq!(tmpl.expand().unwrap(), "Hello, World!");
}

#[test]
fn multi_param() {
    let mut tmpl = Template::from("<<<a>>> + <<<b>>> = <<<c>>>".to_string());

    tmpl! {
        // tmpl, // Template named tmpl can be omitted
        a = "1",
        b = "2",
        c = "3",
    };

    assert_eq!(tmpl.expand().unwrap(), "1 + 2 = 3");
}

#[test]
fn impl_blocks() {
    let mut tmpl = Template::from(
        r#"
>>>>>>>>>> arms
@@@ >>> arms
    "<<<crate_name>>>" => Some(<<<crate_name>>>::exec(data, params).await),
@@@ <<<
"#
        .trim()
        .to_string(),
    );

    tmpl! {
        arms {
            crate_name = "my",
            crate_name = "you",
        }
    }

    let expanded = tmpl.expand().unwrap();
    assert!(expanded.contains(r#""my" => Some(my::exec(data, params).await)"#));
    assert!(expanded.contains(r#""you" => Some(you::exec(data, params).await)"#));
}

#[test]
fn display_block_global_hidden_by_default() {
    let tmpl = Template::from(
        r#"
visible line
??? >>> debug
    hidden line
??? <<<
visible end
"#
        .trim()
        .to_string(),
    );

    let expanded = tmpl.expand().unwrap();
    assert!(expanded.contains("visible line"));
    assert!(expanded.contains("visible end"));
    assert!(!expanded.contains("hidden line"));
}

#[test]
fn display_block_global_shown_via_param() {
    let mut tmpl = Template::from(
        r#"
visible line
??? >>> debug
    shown line
??? <<<
visible end
"#
        .trim()
        .to_string(),
    );

    tmpl! {
        debug = true,
    }

    let expanded = tmpl.expand().unwrap();
    assert!(expanded.contains("visible line"));
    assert!(expanded.contains("visible end"));
    assert!(expanded.contains("shown line"));
}

#[test]
fn display_block_inside_impl_area_hidden_by_default() {
    let mut tmpl = Template::from(
        r#"
>>>>>>>>>> arms
@@@ >>> arms
    <<<crate_name>>> => exec,
??? >>> extra
    <<<crate_name>>> => metrics,
??? <<<
@@@ <<<
"#
        .trim()
        .to_string(),
    );

    tmpl! {
        arms {
            crate_name = "my"
        }
    }

    let expanded = tmpl.expand().unwrap();
    assert!(expanded.contains(r#"my => exec"#));
    assert!(!expanded.contains(r#"my => metrics"#));
}

#[test]
fn display_block_inside_impl_area_shown_by_global_param() {
    let mut tmpl = Template::from(
        r#"
>>>>>>>>>> arms
@@@ >>> arms
    <<<crate_name>>> => exec,
??? >>> extra
    <<<crate_name>>> => metrics,
??? <<<
@@@ <<<
"#
        .trim()
        .to_string(),
    );

    tmpl! {
        extra = true,
        arms {
            crate_name = "my",
            crate_name = "you"
        }
    }

    let expanded = tmpl.expand().unwrap();
    assert!(expanded.contains(r#"my => exec"#));
    assert!(expanded.contains(r#"my => metrics"#));
    assert!(expanded.contains(r#"you => exec"#));
    assert!(expanded.contains(r#"you => metrics"#));
}

#[test]
fn display_block_inside_impl_area_shown_by_arm_param() {
    let mut tmpl = Template::from(
        r#"
>>>>>>>>>> arms
@@@ >>> arms
    <<<crate_name>>> => exec,
??? >>> extra
    <<<crate_name>>> => metrics,
??? <<<
@@@ <<<
"#
        .trim()
        .to_string(),
    );

    tmpl! {
        arms {
            crate_name = "my",
            {
                crate_name = "you",
                extra = true
            }
        }
    }

    let expanded = tmpl.expand().unwrap();
    assert!(expanded.contains(r#"my => exec"#));
    assert!(!expanded.contains(r#"my => metrics"#));
    assert!(expanded.contains(r#"you => exec"#));
    assert!(expanded.contains(r#"you => metrics"#));
}
