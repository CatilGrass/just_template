//! Template struct for storing template strings and their parameters.
//!
//! The template supports two types of parameters:
//! - Simple parameters: key-value pairs used to replace simple placeholders (`<<<key>>>` format) in the template.
//! - Implementation parameters: for implementation blocks (`>>>>>>>>> block_name` and `@@@ >>> block_name` format),
//!   can contain multiple parameter sets, each corresponding to an implementation instance.
//!
//! # Examples
//! ```
//! use just_template::Template;
//!
//! let mut tmpl = Template::from("Hello, <<<name>>>!".to_string());
//! tmpl.insert_param("name".to_string(), "World".to_string());
//! assert_eq!(tmpl.to_string(), "Hello, World!");
//! ```
//!
//! Using the `tmpl_param!` macro makes it easier to add simple parameters:
//! ```
//! use just_template::{Template, tmpl_param};
//!
//! let mut tmpl = Template::from("<<<a>>> + <<<b>>> = <<<c>>>".to_string());
//! tmpl_param!(tmpl, a = 1, b = 2, c = 3);
//! assert_eq!(tmpl.to_string(), "1 + 2 = 3");
//! ```
//!
//! Using the `tmpl!` macro adds implementation block parameters:
//! ```
//! use just_template::{Template, tmpl};
//!
//! let mut tmpl = Template::from("
//! >>>>>>>>>> arms
//! @@@ >>> arms
//!     <<<crate_name>>> => Some(<<<crate_name>>>::exec(data, params).await),
//! @@@ <<<
//! ".trim().to_string());
//! tmpl!(tmpl += {
//!     arms {
//!         (crate_name = "my"),
//!         (crate_name = "you")
//!     }
//! });
//! // Output the expanded template
//! let expanded = tmpl.to_string();
//! assert_eq!(expanded, "
//!     my => Some(my::exec(data, params).await),
//!     you => Some(you::exec(data, params).await),
//! ".trim().to_string());
//! ```
mod template;
pub use template::*; // Re-export template to just_template

pub mod expand;
pub mod test;

#[macro_export]
macro_rules! tmpl_param {
    ($template:ident, $($key:ident = $value:expr),* $(,)?) => {{
        $(
            $template.insert_param(stringify!($key).to_string(), $value.to_string());
        )*
    }};
}

#[macro_export]
macro_rules! tmpl {
    ($template:ident += {
        $($name:ident {
            $(($($key:ident = $value:expr),* $(,)?)),*
            $(,)?
        }),*
    }) => {{
        $(
            let $name = $template.add_impl(stringify!($name).to_string());
            $(
                $name.push({
                    let mut params = std::collections::HashMap::new();
                    $(params.insert(stringify!($key).to_string(), $value.to_string());)*
                    params
                });
            )*
        )*
    }};
}
