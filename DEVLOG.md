# Dev Log

## [Unreleased] - 2023-12-24

### Documentation
- #![doc(html_root_url = "https: //docs.rs/murmur/")]
- Update: lib.rs documentation structure
- Update: `IconKind` enum link in documentation
- Refactor: some crate docs to example .

### Other
- 2023-12-24 09: 56:37
- 2023-12-23 16: 27:45
- 2023-12-23 16: 13:43
- 2023-12-23 07: 57:58
- 2023-12-22 04: 54:11

### Breaking Changes
- Breaking: Changes NfMd Variants are deprecated and will be replaced by their equivalent NfFa Variant in the next release

### Added
- Add: examples folder
- Add: whisper_with_fallback method experimental

### Changed
- Update: README.md content and structure
- Update: icon_map.rs deprecate NfMd icon variants and replace with NfFa equivalents
- Change: whisper or else fallback ok
- Change: whisper_or_else fallback err
- Rename: to whisper_or_else
- Update: makefile

### Configuration
- Allow: deprecated code in Cargo.toml lints settings.

### Removed
- Remove: whisper_or_else

