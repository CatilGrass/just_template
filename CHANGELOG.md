# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-06-29

### Added

- **Display blocks (`??? >>> name` / `??? <<<`)** — new template syntax for
  conditionally included sections. Controlled by `params` key presence: default
  hidden, enabled via `template.insert_param("name", "")`.
- **Per-arm display block control** — each implementation arm can independently
  enable display blocks by including the block name in its own parameter map,
  e.g. `{ crate_name = "you", extra = "" }` enables `??? >>> extra` for that
  arm only.
- **`just_template_macros` proc-macro crate** — new `tmpl!` proc macro replaces
  the old `macro_rules!` macros (`tmpl!`, `tmpl_param!`), supporting:
  - `tmpl!(tmpl, key = val)` and `tmpl! { key = val }` call forms
  - Mixed simple parameters and implementation blocks in one call
  - Braced multi-parameter arms: `{ key = val, key = val }` inside impl blocks
  - Implicit template variable name (defaults to `tmpl` when omitted)
- **Comprehensive test suite** — `test_expand.rs` (direct API tests) and
  `test_macros.rs` (macro syntax tests), covering all features.
- `Template::add_impl` now uses `HashMap::entry(...).or_default()` for brevity.
- Public documentation for `Template` fields.

### Changed

- **Restructured to workspace layout** — root `Cargo.toml` is now a workspace
  root. Library code moved to `just_template/`, proc-macro code to
  `just_template_macros/`.
- **Replaced `macro_rules!` macros with proc macro** — old `tmpl_param!` and
  `tmpl!` (declarative) macros are removed in favor of the new `tmpl!` proc
  macro from `just_template_macros`.
- **`expand.rs` refactored** — added `apply_display_blocks()`, per-arm display
  param merging, migrated from `mem::replace` to `std::mem::take`, and replaced
  `.len() < 1` / `.len() > 0` with `.is_empty()` / `!is_empty()`.

### Removed

- Old deprecated module (`deprecated.rs`) containing the old `macro_rules!`
  macros.
- Standalone test files `test_input.txt` and `test_expect.txt` — replaced by
  inline tests in `test_expand.rs`.
- Root-level `src/lib.rs` and `src/test.rs` — moved into workspace crates.

## [0.1.3] - 2026-03-02

### Fixed

- Fixed the reading logic for implementation block regions in `expand.rs`

## [0.1.2] - 2026-03-02

### Added

- Added `From<&str>` and `From<Cow<'_, str>>` trait implementations for `Template`

## [0.1.1] - 2026-02-28

### Changed

- Refactored crate structure, moved macro definitions to `lib.rs`
- Renamed `src/lib.rs` to `src/template.rs`

## [0.1.0] - 2026-02-27

### Added

- `Template` struct, supporting template strings and two parameter types (simple parameters and implementation block parameters)
- Template syntax: `<<<key>>>` for simple parameter substitution, `>>>>>>>>>> block_name` / `@@@ >>> block_name ... @@@ <<<` for implementation block regions
- `tmpl!` macro: adds implementation block parameters to a template
- `tmpl_param!` macro: adds simple parameters to a template
- `Template::expand()` method: expands the template and returns the final string
- Project initialization

[unreleased]: https://github.com/catilgrass/just_template/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/catilgrass/just_template/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/catilgrass/just_template/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/catilgrass/just_template/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/catilgrass/just_template/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/catilgrass/just_template/releases/tag/v0.1.0
