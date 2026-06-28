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
    ($template:ident, $($name:ident {
        $($key:ident = $value:expr),* $(,)?
    }),* $(,)?) => {{
        $(
            let $name = $template.add_impl(stringify!($name).to_string());
            $(
                $name.push({
                    let mut params = std::collections::HashMap::new();
                    params.insert(stringify!($key).to_string(), $value.to_string());
                    params
                });
            )*
        )*
    }};

    // Old syntax
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
