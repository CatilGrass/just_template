//! A template engine for code generation.
//!
//! # Template syntax
//!
//! ## Simple parameters — `<<<key>>>`
//!
//! The most basic substitution. Replaced with the value set via
//! [`Template::insert_param`] or the [`tmpl!`] macro.
//!
//! ```rust
//! # use just_template::Template;
//! let mut t = Template::from("Hello, <<<name>>>!".to_string());
//! t.insert_param("name".to_string(), "World".to_string());
//! assert_eq!(t.expand().unwrap(), "Hello, World!");
//! ```
//!
//! ## Implementation blocks
//!
//! **Syntax:** `>>>>>>>>>> name` / `@@@ >>> name ... @@@ <<<`
//!
//! Template sections that can be instantiated multiple times with different
//! parameter sets. Each instantiation is called an "arm".
//!
//! ```rust
//! # use just_template::Template;
//! let mut t = Template::from(r#"
//! >>>>>>>>>> match_arms
//! @@@ >>> match_arms
//!     <<<value>>> => println!("<<<value>>>"),
//! @@@ <<<
//! "#.trim().to_string());
//!
//! let arms = t.add_impl("match_arms".to_string());
//! arms.push(std::collections::HashMap::from([
//!     ("value".to_string(), "a".to_string()),
//! ]));
//! arms.push(std::collections::HashMap::from([
//!     ("value".to_string(), "b".to_string()),
//! ]));
//!
//! let out = t.expand().unwrap();
//! assert!(out.contains(r#"a => println!("a")"#));
//! assert!(out.contains(r#"b => println!("b")"#));
//! ```
//!
//! ## Display blocks
//!
//! **Syntax:** `??? >>> name` / `??? <<<`
//!
//! Conditionally included sections. Hidden by default; shown when a parameter
//! with the same name exists in the parameter map.
//!
//! ```rust
//! # use just_template::Template;
//! // Without enabling, the block is omitted
//! let t = Template::from(r#"
//! visible
//! ??? >>> debug
//!     hidden by default
//! ??? <<<
//! "#.trim().to_string());
//! assert_eq!(t.expand().unwrap(), "visible");
//!
//! // Enable by inserting a param with the block name
//! let mut t = Template::from(r#"
//! visible
//! ??? >>> debug
//!     hidden by default
//! ??? <<<
//! "#.trim().to_string());
//! t.insert_param("debug".to_string(), "".to_string());
//! let out = t.expand().unwrap();
//! assert!(out.contains("hidden by default"));
//! ```
//!
//! Display blocks also work inside implementation blocks, and can be controlled
//! per arm by including the block name in that arm's parameter map:
//!
//! ```rust
//! # use just_template::Template;
//! let mut t = Template::from(r#"
//! >>>>>>>>>> arms
//! @@@ >>> arms
//!     <<<name>>>
//! ??? >>> extra
//!     <<<name>>>.extra()
//! ??? <<<
//! @@@ <<<
//! "#.trim().to_string());
//!
//! let arms = t.add_impl("arms".to_string());
//! // Arm 1: no "extra" → display block hidden
//! arms.push(std::collections::HashMap::from([
//!     ("name".to_string(), "foo".to_string()),
//! ]));
//! // Arm 2: has "extra" → display block shown for this arm only
//! arms.push(std::collections::HashMap::from([
//!     ("name".to_string(), "bar".to_string()),
//!     ("extra".to_string(), "".to_string()),
//! ]));
//!
//! let out = t.expand().unwrap();
//! assert!(out.contains("foo"));
//! assert!(!out.contains("foo.extra()"));
//! assert!(out.contains("bar"));
//! assert!(out.contains("bar.extra()"));
//! ```
//!
//! # The `tmpl!` macro
//!
//! The [`tmpl!`](macro.tmpl.html) proc macro provides a concise syntax for
//! setting both simple parameters and implementation blocks.
//!
//! ```rust
//! # use just_template::{Template, tmpl};
//! let mut t = Template::from(r#"
//! >>>>>>>>>> arms
//! @@@ >>> arms
//!     <<<crate_name>>> => Some(<<<crate_name>>>::exec(data, params).await),
//! @@@ <<<
//! "#.trim().to_string());
//!
//! # let param: i32 = 1; // dummy
//! tmpl!(t,
//!     func_name = "my_func",
//!     arms {
//!         crate_name = "my",
//!         {
//!             crate_name = "you",
//!             extra = ""
//!         }
//!     }
//! );
//! ```
//!
//! When the template variable is named `tmpl`, it can be omitted:
//!
//! ```rust
//! # use just_template::{Template, tmpl};
//! let mut tmpl = Template::from("<<<a>>> + <<<b>>> = <<<c>>>".to_string());
//!
//! tmpl! {
//!     a = "1",
//!     b = "2",
//!     c = "3",
//! };
//!
//! assert_eq!(tmpl.expand().unwrap(), "1 + 2 = 3");
//! ```

mod expand;

mod template;
pub use template::*;

/// Re-exports the `tmpl!` macro from `just_template_macros`.
///
/// This macro provides a concise syntax for setting template parameters
/// and implementation blocks. See the [module-level documentation](index.html#the-tmpl-macro)
/// for usage examples.
pub use just_template_macros::tmpl;

#[cfg(test)]
pub mod test_expand;

#[cfg(test)]
pub mod test_macros;
