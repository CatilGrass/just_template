use std::collections::HashMap;

use just_fmt::snake_case;

use crate::template::Template;

const DISPLAY_BLOCK_BEGIN: &str = "??? >>> ";
const DISPLAY_BLOCK_END: &str = "??? <<<";

const IMPL_AREA_BEGIN: &str = "@@@ >>> ";
const IMPL_AREA_END: &str = "@@@ <<<";

const IMPL_BEGIN: &str = ">>>>>>>>>>";

const PARAM_BEGIN: &str = "<<<";
const PARAM_BEND: &str = ">>>";

impl Template {
    pub fn expand(mut self) -> Option<String> {
        // Extract template text
        let expanded = std::mem::take(&mut self.template_str);

        let (expanded, impl_areas) = read_impl_areas(expanded)?;
        let expanded = apply_impls(&self, expanded, impl_areas)?;
        let expanded = apply_display_blocks(&self.params, expanded);
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
            if current_area_name.is_empty() {
                return None;
            }

            // Submit Impl Area
            let name = std::mem::take(&mut current_area_name);
            impl_areas.insert(name, current_area_codes.join("\n"));
            current_area_codes.clear();
            continue;
        }

        // Implementation block start
        if trimmed_line.starts_with(IMPL_AREA_BEGIN) {
            // If the current ImplArea name is not empty, a block is already active.
            // Nesting is not allowed, so matching fails and exits early.
            if !current_area_name.is_empty() {
                return None;
            }

            // Get a snake_case name
            let snake_name = snake_case!(line.trim_start_matches(IMPL_AREA_BEGIN).trim());
            current_area_name = snake_name;

            // Continue to next line
            continue;
        }

        // During implementation block
        if !current_area_name.is_empty() {
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
            let mut applied = impl_area_template.clone();

            // Merge global params with arm-specific params for display block check
            let mut display_params = template.params.clone();
            for (k, v) in item {
                display_params.insert(k.clone(), v.clone());
            }
            applied = apply_display_blocks(&display_params, applied);

            // Extract parameters
            for (param_name, param_value) in item {
                // Apply parameter
                applied = applied.replace(
                    &format!("{}{}{}", PARAM_BEGIN, param_name, PARAM_BEND),
                    param_value,
                );
            }

            // Add applied template
            impled_area_code_applied.push(applied);
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

            if !impled_code.is_empty() {
                applied_content += "\n";
                applied_content += impled_code.join("\n").as_str();
            }
        } else {
            // Other content directly appended
            applied_content += "\n";
            applied_content += line;
        }
    }

    Some(applied_content)
}

/// Process display blocks (`??? >>> name` / `??? <<<`).
///
/// If `params` contains a key matching the block name, the block content is
/// included (with markers removed). Otherwise the entire block is omitted.
fn apply_display_blocks(params: &HashMap<String, String>, content: String) -> String {
    let mut result = String::new();
    let lines: Vec<&str> = content.split("\n").collect();
    let mut i = 0;
    let mut first = true;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.starts_with(DISPLAY_BLOCK_BEGIN) {
            let block_name = trimmed.trim_start_matches(DISPLAY_BLOCK_BEGIN).trim();
            let show = params.contains_key(block_name);
            i += 1;

            while i < lines.len() && !lines[i].trim().starts_with(DISPLAY_BLOCK_END) {
                if show {
                    if !first {
                        result += "\n";
                    }
                    result += lines[i];
                    first = false;
                }
                i += 1;
            }
        } else if !trimmed.starts_with(DISPLAY_BLOCK_END) {
            if !first {
                result += "\n";
            }
            result += line;
            first = false;
        }

        i += 1;
    }

    result
}

fn apply_param(template: &Template, content: String) -> Option<String> {
    let mut content = content;
    for (k, v) in template.params.iter() {
        content = content.replace(&format!("{}{}{}", PARAM_BEGIN, k, PARAM_BEND), v);
    }
    Some(content)
}
