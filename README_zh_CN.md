# just_template

> 用于代码生成的模板引擎。

## 模板语法

### 简单参数 — `<<<key>>>`

最基本的替换方式。通过 [`Template::insert_param`] 或 [`tmpl!`] 宏设置的值来替换。

```rust
# use just_template::Template;
let mut t = Template::from("Hello, <<<name>>>!".to_string());
t.insert_param("name".to_string(), "World".to_string());
assert_eq!(t.expand().unwrap(), "Hello, World!");
```

### 实现块

**语法：** `>>>>>>>>>> name` / `@@@ >>> name ... @@@ <<<`

可以使用不同参数集多次实例化的模板片段。每次实例化称为一个"分支"。

```rust
# use just_template::Template;
let mut t = Template::from(r#"
>>>>>>>>>> match_arms
@@@ >>> match_arms
    <<<value>>> => println!("<<<value>>>"),
@@@ <<<
"#.trim().to_string());

let arms = t.add_impl("match_arms".to_string());
arms.push(std::collections::HashMap::from([
    ("value".to_string(), "a".to_string()),
]));
arms.push(std::collections::HashMap::from([
    ("value".to_string(), "b".to_string()),
]));

let out = t.expand().unwrap();
assert!(out.contains(r#"a => println!("a")"#));
assert!(out.contains(r#"b => println!("b")"#));
```

### 显示块

**语法：** `??? >>> name` / `??? <<<`

条件包含的片段。默认隐藏；当参数映射中存在同名参数时显示。

```rust
# use just_template::Template;
// 不启用时，块被省略
let t = Template::from(r#"
visible
??? >>> debug
    hidden by default
??? <<<
"#.trim().to_string());
assert_eq!(t.expand().unwrap(), "visible");

// 通过插入与块名同名的参数来启用
let mut t = Template::from(r#"
visible
??? >>> debug
    hidden by default
??? <<<
"#.trim().to_string());
t.insert_param("debug".to_string(), "".to_string());
let out = t.expand().unwrap();
assert!(out.contains("hidden by default"));
```

显示块也可以在实现块内部工作，并且可以通过在分支的参数映射中包含块名来对每个分支进行控制：

```rust
# use just_template::Template;
let mut t = Template::from(r#"
>>>>>>>>>> arms
@@@ >>> arms
    <<<name>>>
??? >>> extra
    <<<name>>>.extra()
??? <<<
@@@ <<<
"#.trim().to_string());

let arms = t.add_impl("arms".to_string());
// 分支 1：没有 "extra" → 显示块隐藏
arms.push(std::collections::HashMap::from([
    ("name".to_string(), "foo".to_string()),
]));
// 分支 2：有 "extra" → 仅此分支显示显示块
arms.push(std::collections::HashMap::from([
    ("name".to_string(), "bar".to_string()),
    ("extra".to_string(), "".to_string()),
]));

let out = t.expand().unwrap();
assert!(out.contains("foo"));
assert!(!out.contains("foo.extra()"));
assert!(out.contains("bar"));
assert!(out.contains("bar.extra()"));
```

### `tmpl!` 宏

[`tmpl!`](macro.tmpl.html) 过程宏提供了一种简洁的语法，用于同时设置简单参数和实现块。

```rust
# use just_template::{Template, tmpl};
let mut t = Template::from(r#"
>>>>>>>>>> arms
@@@ >>> arms
    <<<crate_name>>> => Some(<<<crate_name>>>::exec(data, params).await),
@@@ <<<
"#.trim().to_string());

# let param: i32 = 1; // dummy
tmpl!(t,
    func_name = "my_func",
    arms {
        crate_name = "my",
        {
            crate_name = "you",
            extra = ""
        }
    }
);
```

当模板变量名为 `tmpl` 时，可以省略它：

```rust
# use just_template::{Template, tmpl};
let mut tmpl = Template::from("<<<a>>> + <<<b>>> = <<<c>>>".to_string());

tmpl! {
    a = "1",
    b = "2",
    c = "3",
};

assert_eq!(tmpl.expand().unwrap(), "1 + 2 = 3");
```

## 安装

将以下内容添加到您的 `Cargo.toml`：

```toml
[dependencies]
just_template = "0.2"
```

## 许可证

本项目采用 MIT 和 Apache 2.0 双许可证。
详情请参阅 LICENSE 文件。
