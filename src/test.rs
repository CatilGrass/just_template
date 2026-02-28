#[cfg(test)]
mod tests {
    use crate::{template::Template, tmpl, tmpl_param};

    #[test]
    fn expand() {
        let input = std::fs::read_to_string("./src/test_input.txt")
            .unwrap()
            .trim()
            .to_string();
        let expect = std::fs::read_to_string("./src/test_expect.txt")
            .unwrap()
            .trim()
            .to_string();

        let mut tmpl = Template::from(input);
        tmpl_param!(tmpl, func_name = "my_func");
        tmpl!(tmpl += {
            arms {
                (crate_name = "my"),
                (crate_name = "you")
            }
        });

        let expanded = tmpl.expand().unwrap();
        assert_eq!(expanded, expect);
    }
}
