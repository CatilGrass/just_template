#![doc = include_str!("./lib.md")]

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
