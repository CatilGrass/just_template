//! Template struct for storing template strings and their parameters.

mod expand;

mod template;
pub use template::*; // Re-export template

pub use just_template_macros::*; // Re-export macros

#[cfg(test)]
pub mod test_expand;

#[cfg(test)]
pub mod test_macros;
