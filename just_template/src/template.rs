use std::collections::HashMap;

/// Template struct, used to represent a template composed of a template string and parameters.
///
/// # Fields
/// - `template_str` - Template string containing placeholders.
/// - `params` - Normal parameters, key-value mappings.
/// - `impl_params` - Implementation block parameters, where each implementation block name corresponds to a list of parameter maps (each parameter map is a key-value mapping).
#[derive(Default, Clone)]
pub struct Template {
    /// Template string containing placeholders such as `{{param_name}}`.
    pub(crate) template_str: String,
    /// Normal parameters, keyed by parameter name with values as parameter values.
    pub(crate) params: HashMap<String, String>,
    /// Implementation block parameters, keyed by implementation block name with values as a list of parameter maps.
    pub(crate) impl_params: HashMap<String, Vec<HashMap<String, String>>>,
}

impl Template {
    /// Add a parameter
    pub fn insert_param(&mut self, name: String, value: String) {
        self.params.insert(name, value);
    }

    /// Add an implementation block and return a HashMap to set its parameters
    pub fn add_impl(&mut self, impl_name: String) -> &mut Vec<HashMap<String, String>> {
        self.impl_params.entry(impl_name).or_default()
    }
}

impl From<String> for Template {
    fn from(s: String) -> Self {
        Template {
            template_str: s,
            ..Default::default()
        }
    }
}

impl<'a> From<&'a str> for Template {
    fn from(s: &'a str) -> Self {
        Template {
            template_str: s.to_string(),
            ..Default::default()
        }
    }
}

impl<'a> From<std::borrow::Cow<'a, str>> for Template {
    fn from(s: std::borrow::Cow<'a, str>) -> Self {
        Template {
            template_str: s.into_owned(),
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cloned = self.clone();
        write!(f, "{}", cloned.expand().unwrap_or_default())
    }
}
