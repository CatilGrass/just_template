use std::collections::HashMap;

use crate::Template;

#[test]
fn basic_param() {
    let mut tmpl = Template::from("Hello, <<<name>>>!".to_string());
    tmpl.insert_param("name".to_string(), "World".to_string());
    assert_eq!(tmpl.expand().unwrap(), "Hello, World!");
}

#[test]
fn multi_param() {
    let mut tmpl = Template::from("<<<a>>> + <<<b>>> = <<<c>>>".to_string());
    tmpl.insert_param("a".to_string(), "1".to_string());
    tmpl.insert_param("b".to_string(), "2".to_string());
    tmpl.insert_param("c".to_string(), "3".to_string());
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

    let arms = tmpl.add_impl("arms".to_string());
    arms.push(HashMap::from([(
        "crate_name".to_string(),
        "my".to_string(),
    )]));
    arms.push(HashMap::from([(
        "crate_name".to_string(),
        "you".to_string(),
    )]));

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

    tmpl.insert_param("debug".to_string(), "".to_string());

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

    let arms = tmpl.add_impl("arms".to_string());
    arms.push(HashMap::from([(
        "crate_name".to_string(),
        "my".to_string(),
    )]));

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

    // Enable via global param — shows for ALL arms
    tmpl.insert_param("extra".to_string(), "".to_string());

    let arms = tmpl.add_impl("arms".to_string());
    arms.push(HashMap::from([(
        "crate_name".to_string(),
        "my".to_string(),
    )]));
    arms.push(HashMap::from([(
        "crate_name".to_string(),
        "you".to_string(),
    )]));

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

    let arms = tmpl.add_impl("arms".to_string());
    // Arm 1: no "extra" → hidden
    arms.push(HashMap::from([(
        "crate_name".to_string(),
        "my".to_string(),
    )]));
    // Arm 2: has "extra" → shown for this arm only
    arms.push(HashMap::from([
        ("crate_name".to_string(), "you".to_string()),
        ("extra".to_string(), "".to_string()),
    ]));

    let expanded = tmpl.expand().unwrap();
    assert!(expanded.contains(r#"my => exec"#));
    assert!(!expanded.contains(r#"my => metrics"#));
    assert!(expanded.contains(r#"you => exec"#));
    assert!(expanded.contains(r#"you => metrics"#));
}
