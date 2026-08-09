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

#[test]
fn display_block_nested_both_shown() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
??? >>> B
B content
??? <<<
??? <<<
"#
        .trim()
        .to_string(),
    );

    tmpl.insert_param("A".to_string(), "".to_string());
    tmpl.insert_param("B".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== both shown ===\n{:?}\n", expanded);
    assert!(expanded.contains("B content"));
    assert!(!expanded.contains("??? >>> B"));
}

#[test]
fn display_block_nested_outer_shown_inner_hidden() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
??? >>> B
B content
??? <<<
??? <<<
"#
        .trim()
        .to_string(),
    );

    // Only A enabled, B should be hidden by its own param check
    tmpl.insert_param("A".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== outer shown, inner hidden ===\n{:?}\n", expanded);
    assert!(!expanded.contains("B content"));
}

#[test]
fn display_block_nested_outer_hidden() {
    let tmpl = Template::from(
        r#"
??? >>> A
??? >>> B
B content
??? <<<
??? <<<
"#
        .trim()
        .to_string(),
    );

    let expanded = tmpl.expand().unwrap();
    println!("=== outer hidden ===\n{:?}\n", expanded);
    assert!(!expanded.contains("B content"));
}

#[test]
fn display_block_nested_inner_shown_outer_hidden() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
??? >>> B
B content
??? <<<
??? <<<
"#
        .trim()
        .to_string(),
    );

    // Only B enabled; A is off so the whole subtree must be hidden
    tmpl.insert_param("B".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== inner shown, outer hidden ===\n{:?}\n", expanded);
    assert!(!expanded.contains("B content"));
    assert!(!expanded.contains("??? >>>"));
}

#[test]
fn display_block_three_levels_all_shown() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
  ??? >>> B
    ??? >>> C
      C content
    ??? <<<
  ??? <<<
??? <<<
"#
        .trim()
        .to_string(),
    );

    tmpl.insert_param("A".to_string(), "".to_string());
    tmpl.insert_param("B".to_string(), "".to_string());
    tmpl.insert_param("C".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== three levels all shown ===\n{:?}\n", expanded);
    assert!(expanded.contains("C content"));
    assert!(!expanded.contains("??? >>>"));
    assert!(!expanded.contains("??? <<<"));
}

#[test]
fn display_block_three_levels_middle_hidden() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
  ??? >>> B
    ??? >>> C
      C content
    ??? <<<
  ??? <<<
??? <<<
"#
        .trim()
        .to_string(),
    );

    // A and C on, B off -> C subtree must be hidden
    tmpl.insert_param("A".to_string(), "".to_string());
    tmpl.insert_param("C".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== three levels, middle hidden ===\n{:?}\n", expanded);
    assert!(!expanded.contains("C content"));
    assert!(!expanded.contains("??? >>>"));
}

#[test]
fn display_block_three_levels_inner_only() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
  ??? >>> B
    ??? >>> C
      C content
    ??? <<<
  ??? <<<
??? <<<
"#
        .trim()
        .to_string(),
    );

    // Only the innermost is on, but its ancestors are off
    tmpl.insert_param("C".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== three levels, inner only ===\n{:?}\n", expanded);
    assert!(!expanded.contains("C content"));
    assert!(!expanded.contains("??? >>>"));
}

#[test]
fn display_block_siblings_both_shown() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
  before
  ??? >>> B
    B content
  ??? <<<
  ??? >>> C
    C content
  ??? <<<
  after
??? <<<
"#
        .trim()
        .to_string(),
    );

    tmpl.insert_param("A".to_string(), "".to_string());
    tmpl.insert_param("B".to_string(), "".to_string());
    tmpl.insert_param("C".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== siblings both shown ===\n{:?}\n", expanded);
    assert!(expanded.contains("before"));
    assert!(expanded.contains("B content"));
    assert!(expanded.contains("C content"));
    assert!(expanded.contains("after"));
    assert!(!expanded.contains("??? >>>"));
    assert!(!expanded.contains("??? <<<"));
}

#[test]
fn display_block_siblings_one_hidden() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
  before
  ??? >>> B
    B content
  ??? <<<
  ??? >>> C
    C content
  ??? <<<
  after
??? <<<
"#
        .trim()
        .to_string(),
    );

    // B on, C off -> B shows, C is dropped
    tmpl.insert_param("A".to_string(), "".to_string());
    tmpl.insert_param("B".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== siblings one hidden ===\n{:?}\n", expanded);
    assert!(expanded.contains("before"));
    assert!(expanded.contains("B content"));
    assert!(!expanded.contains("C content"));
    assert!(expanded.contains("after"));
    assert!(!expanded.contains("??? >>>"));
}

#[test]
fn display_block_siblings_outer_hidden() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
  before
  ??? >>> B
    B content
  ??? <<<
  ??? >>> C
    C content
  ??? <<<
  after
??? <<<
"#
        .trim()
        .to_string(),
    );

    // A off, B and C on -> everything inside A must be hidden
    tmpl.insert_param("B".to_string(), "".to_string());
    tmpl.insert_param("C".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== siblings outer hidden ===\n{:?}\n", expanded);
    assert!(!expanded.contains("before"));
    assert!(!expanded.contains("B content"));
    assert!(!expanded.contains("C content"));
    assert!(!expanded.contains("after"));
}

#[test]
fn display_block_nested_with_surrounding_text() {
    let mut tmpl = Template::from(
        r#"
head line
??? >>> A
  ??? >>> B
    B content
  ??? <<<
  A footer
??? <<<
tail line
"#
        .trim()
        .to_string(),
    );

    tmpl.insert_param("A".to_string(), "".to_string());
    tmpl.insert_param("B".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== nested with surrounding text ===\n{:?}\n", expanded);
    assert!(expanded.contains("head line"));
    assert!(expanded.contains("B content"));
    assert!(expanded.contains("A footer"));
    assert!(expanded.contains("tail line"));
    assert!(!expanded.contains("??? >>>"));
    assert!(!expanded.contains("??? <<<"));
}

#[test]
fn display_block_deep_nested_level4() {
    let mut tmpl = Template::from(
        r#"
??? >>> A
  ??? >>> B
    ??? >>> C
      ??? >>> D
        D content
      ??? <<<
    ??? <<<
  ??? <<<
??? <<<
"#
        .trim()
        .to_string(),
    );

    // A and D on, but C is off -> D content must stay hidden
    tmpl.insert_param("A".to_string(), "".to_string());
    tmpl.insert_param("D".to_string(), "".to_string());

    let expanded = tmpl.expand().unwrap();
    println!("=== deep nested level 4 ===\n{:?}\n", expanded);
    assert!(!expanded.contains("D content"));
    assert!(!expanded.contains("??? >>>"));
}

#[test]
fn display_block_nested_inside_impl_area() {
    let mut tmpl = Template::from(
        r#"
>>>>>>>>>> arms
@@@ >>> arms
  <<<crate_name>>> => exec,
??? >>> A
  ??? >>> B
    <<<crate_name>>> => nested,
  ??? <<<
??? <<<
@@@ <<<
"#
        .trim()
        .to_string(),
    );

    tmpl.insert_param("A".to_string(), "".to_string());
    tmpl.insert_param("B".to_string(), "".to_string());

    let arms = tmpl.add_impl("arms".to_string());
    arms.push(HashMap::from([(
        "crate_name".to_string(),
        "my".to_string(),
    )]));

    let expanded = tmpl.expand().unwrap();
    println!("=== nested inside impl area ===\n{:?}\n", expanded);
    assert!(expanded.contains(r#"my => exec"#));
    assert!(expanded.contains(r#"my => nested"#));
    assert!(!expanded.contains("??? >>>"));
}

#[test]
fn display_block_nested_inside_impl_area_arm_param() {
    let mut tmpl = Template::from(
        r#"
>>>>>>>>>> arms
@@@ >>> arms
  <<<crate_name>>> => exec,
??? >>> A
  ??? >>> B
    <<<crate_name>>> => nested,
  ??? <<<
??? <<<
@@@ <<<
"#
        .trim()
        .to_string(),
    );

    let arms = tmpl.add_impl("arms".to_string());
    // Arm 1: enables A and B -> nested line shown for this arm only
    arms.push(HashMap::from([
        ("crate_name".to_string(), "my".to_string()),
        ("A".to_string(), "".to_string()),
        ("B".to_string(), "".to_string()),
    ]));
    // Arm 2: no A/B -> nested line hidden
    arms.push(HashMap::from([(
        "crate_name".to_string(),
        "you".to_string(),
    )]));

    let expanded = tmpl.expand().unwrap();
    println!("=== nested inside impl area, arm param ===\n{:?}\n", expanded);
    assert!(expanded.contains(r#"my => exec"#));
    assert!(expanded.contains(r#"my => nested"#));
    assert!(expanded.contains(r#"you => exec"#));
    assert!(!expanded.contains(r#"you => nested"#));
    assert!(!expanded.contains("??? >>>"));
}
