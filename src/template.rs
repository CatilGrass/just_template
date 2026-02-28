use std::collections::HashMap;

pub mod expand;
pub mod test;

#[derive(Default, Clone)]
pub struct Template {
    pub(crate) template_str: String,
    pub(crate) params: HashMap<String, String>,
    pub(crate) impl_params: HashMap<String, Vec<HashMap<String, String>>>,
}

impl Template {
    /// Add a parameter
    pub fn insert_param(&mut self, name: String, value: String) {
        self.params.insert(name, value);
    }

    /// Add an implementation block and return a HashMap to set its parameters
    pub fn add_impl(&mut self, impl_name: String) -> &mut Vec<HashMap<String, String>> {
        self.impl_params
            .entry(impl_name)
            .or_insert_with(|| Vec::<HashMap<String, String>>::new())
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

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cloned = self.clone();
        write!(f, "{}", cloned.expand().unwrap_or_default())
    }
}

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
