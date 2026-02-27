use std::{collections::HashMap, mem::replace};

use just_fmt::snake_case;

use crate::Template;

const IMPL_AREA_BEGIN: &str = "@@@ >>> ";
const IMPL_AREA_END: &str = "@@@ <<<";

const IMPL_BEGIN: &str = ">>>>>>>>>>";

const PARAM_BEGIN: &str = "<<<";
const PARAM_BEND: &str = ">>>";

impl Template {
    pub fn expand(mut self) -> Option<String> {
        // Extract template text
        let expanded = replace(&mut self.template_str, String::default());

        let (expanded, impl_areas) = read_impl_areas(expanded)?;
        let expanded = apply_impls(&self, expanded, impl_areas)?;
        let expanded = apply_param(&self, expanded)?;
        Some(expanded.trim().to_string())
    }
}

/// Read all ImplAreas (HashMap<Name, Codes>)
fn read_impl_areas(content: String) -> Option<(String, HashMap<String, String>)> {
    let mut striped_content = String::new();
    let mut impl_areas: HashMap<String, String> = HashMap::new();

    let mut current_area_name = String::default();
    let mut current_area_codes: Vec<String> = Vec::new();

    for line in content.split("\n") {
        let trimmed_line = line.trim();

        // Implementation block end
        if trimmed_line.starts_with(IMPL_AREA_END) {
            // If the current ImplArea name length is less than 1, it means no block is being matched,
            // so matching fails, exit early
            if current_area_name.len() < 1 {
                return None;
            }

            // Submit Impl Area
            let name = replace(&mut current_area_name, String::default());
            impl_areas.insert(name, current_area_codes.join("\n"));
            current_area_codes.clear();
            continue;
        }

        // Implementation block start
        if trimmed_line.starts_with(IMPL_AREA_BEGIN) {
            // If the current ImplArea name length is greater than 0, it means we are already inside a block,
            // since nesting is not allowed, matching fails, exit early
            if current_area_name.len() > 0 {
                return None;
            }

            // Get a snake_case name
            let snake_name = snake_case!(line.trim_start_matches(IMPL_AREA_BEGIN).trim());
            current_area_name = snake_name;

            // Continue to next line
            continue;
        }

        // During implementation block
        if current_area_name.len() > 0 {
            // Add to current block code
            current_area_codes.push(line.to_string());
            continue;
        } else {
            // Add to remaining content
            striped_content += "\n";
            striped_content += line;
        }
    }

    Some((striped_content, impl_areas))
}

/// Apply Template parameters to implementation block areas
fn apply_impls(
    template: &Template,
    content: String,
    impl_areas: HashMap<String, String>,
) -> Option<String> {
    let mut applied_content = String::new();

    let mut impled_areas: HashMap<String, Vec<String>> = HashMap::new();
    for (impl_area_name, impl_area_template) in impl_areas {
        // Get user-provided parameters
        let impl_items = template.impl_params.get(&impl_area_name);

        // No parameters, return early
        let Some(impl_items) = impl_items else {
            impled_areas.insert(impl_area_name, Vec::new());
            continue;
        };

        let mut impled_area_code_applied = Vec::new();

        // Split items
        for item in impl_items {
            // Get base template
            let mut template = impl_area_template.clone();

            // Extract parameters
            for (param_name, param_value) in item {
                // Apply parameter
                template = template.replace(
                    &format!("{}{}{}", PARAM_BEGIN, param_name, PARAM_BEND),
                    param_value,
                );
            }

            // Add applied template
            impled_area_code_applied.push(template);
        }

        impled_areas.insert(impl_area_name, impled_area_code_applied);
    }

    for line in content.split("\n") {
        let trimmed_line = line.trim();

        // Recognize implementation line
        if trimmed_line.starts_with(IMPL_BEGIN) {
            let impl_name = snake_case!(trimmed_line.trim_start_matches(IMPL_BEGIN).trim());

            // Try to get implementation code block
            let Some(impled_code) = impled_areas.get(&impl_name) else {
                continue;
            };

            applied_content += "\n";
            applied_content += impled_code.join("\n").as_str();
        } else {
            // Other content directly appended
            applied_content += "\n";
            applied_content += line;
        }
    }

    Some(applied_content)
}

fn apply_param(template: &Template, content: String) -> Option<String> {
    let mut content = content;
    for (k, v) in template.params.iter() {
        content = content.replace(&format!("{}{}{}", PARAM_BEGIN, k, PARAM_BEND), v);
    }
    Some(content)
}
