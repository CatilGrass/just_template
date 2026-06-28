# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-06-29

### Added

- Simplified `tmpl!` macro syntax: supports `tmpl!(tmpl, block_name { key = val, ... })` form, removing the `+=` operator and parentheses around each parameter group
- Old syntax `tmpl!(tmpl += { ... })` remains backward compatible

### Changed

- Documentation and test code unified to use the simplified `tmpl!` syntax

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
