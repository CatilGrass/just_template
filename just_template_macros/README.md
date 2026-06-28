# just_template_macros

> Proc-macro crate for [`just_template`](https://crates.io/crates/just_template).
> Provides the `tmpl!` macro.

## `tmpl!` macro

A unified macro for setting both simple parameters and implementation blocks
on a [`Template`](https://docs.rs/just_template).

### Simple parameters

```rust
use just_template::{Template, tmpl};

let mut tmpl = Template::from("Hello, <<<name>>>!".to_string());

tmpl!(tmpl, name = "World");
// or, when the variable is named `tmpl`:
tmpl! { name = "World" };

assert_eq!(tmpl.expand().unwrap(), "Hello, World!");
```

### Implementation blocks with arms

```rust
use just_template::{Template, tmpl};

let mut tmpl = Template::from(r#"
>>>>>>>>>> arms
@@@ >>> arms
    <<<value>>> => println!("<<<value>>>"),
@@@ <<<
"#.trim().to_string());

tmpl! {
    arms {
        value = "a",
        value = "b",
    }
};

let out = tmpl.expand().unwrap();
// out contains:
//   a => println!("a"),
//   b => println!("b"),
```

### Mixed with multi-param arms

```rust
use just_template::{Template, tmpl};

let mut tmpl = Template::from(r#"
>>>>>>>>>> arms
@@@ >>> arms
    <<<name>>>: <<<value>>>
??? >>> extra
    <<<name>>>.extra()
??? <<<
@@@ <<<
"#.trim().to_string());

tmpl! {
    arms {
        name = "foo", value = "1",
        {
            name = "bar",
            value = "2",
            extra = ""  // enables the `extra` display block for this arm
        }
    }
};

let out = tmpl.expand().unwrap();
// out contains:
//   foo: 1
//   bar: 2
//   bar.extra()
```

### Call forms

| Form | Example | Template variable |
|---|---|---|
| Explicit | `tmpl!(tmpl, key = val)` | First argument |
| Implicit | `tmpl! { key = val }` | Defaults to `tmpl` |

Both forms accept the same body syntax: simple params (`key = val`),
implementation blocks (`name { arms }`), or a mix of both.

For more details, see the [`just_template` documentation](https://docs.rs/just_template).
