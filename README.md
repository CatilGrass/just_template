# just_template

> a tool for code gen via templates

## Template File Writing Rules

`just_template` gens repetitive code using three core concepts: `impl_mark`, `impl_area`, and `param`.

1. impl_mark
   Mark an "impl point" with a line starting with 10 `>` chars: `>>>>>>>>>> NAME`.
   Used for positioning. The system extracts the matching `impl_area` content and expands it here.

2. impl_area
   Declare a reusable code template:
   `@@@ NAME >>>`
   [template content]
   `@@@ <<<`
   Inside, use param placeholders like `<<<PARAM>>>`.
   When adding an impl via cmd (e.g., `insert_impl!`), the system copies the area, replaces params, and appends to the impl_mark.

3. param
   - Params outside an impl_area are replaced globally.
   - Params inside are replaced per-impl when generating.

Example:

Template:
```rust
// Auto generated
use std::collections::HashMap;

pub async fn my_func(
    name: &str,
    data: &[u8],
    params: &HashMap<String, String>,
) -> Option<Result<Vec<u32>, std::io::Error>> {
    match name {
>>>>>>>>>> arms
@@@ >>> arms
        "<<<crate_name>>>" => Some(<<<crate_name>>>::exec(data, params).await),
@@@ <<<
        _ => None,
    }
}
```

Run cmds:
```rust
tmpl!(tmpl += {
    arms {
        (crate_name = "my"),
        (crate_name = "you")
    }
});
```

The `arms` impl_area becomes:
```rust
        "my" => Some(my::exec(data, params).await),
        "you" => Some(you::exec(data, params).await),
```

Final expanded code:
```rust
// Auto generated
use std::collections::HashMap;

pub async fn my_func(
    name: &str,
    data: &[u8],
    params: &HashMap<String, String>,
) -> Option<Result<Vec<u32>, std::io::Error>> {
    match name {
        "my" => Some(my::exec(data, params).await),
        "you" => Some(you::exec(data, params).await),
        _ => None,
    }
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
just_template = "0.1"
```

## License

This project is dual-licensed under MIT and Apache 2.0.
See the LICENSE file for details.
