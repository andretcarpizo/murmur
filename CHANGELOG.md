# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [[Unreleased]] - 2023-12-25

### Added
- IconKind::
    -  NfFaFolder
    -  NfFaFolderOpen
    -  NfFaeEqual

### Removed
- Remove: DEVLOG.md
- Remove: mutex lock on color_fn
- Remove: whisper_or_else

### Breaking Changes
- NfMd Variants are deprecated and will be replaced by their equivalent NfFa Variant in the next release
  - NfMdGreaterThan
  - NfMdLessThan
  - NfMdEqual
  - NfMdThumbsUp
  - NfMdThumbsDown
  - NfMdFolder
  - NfMdFolderOpen

- Add: IconKind enum has been marked #[non_exhaustive].
  - Pattern-matching on it outside of its crate must now include a wildcard pattern like `_`, or it will fail to compile.

### Configuration
- Allow: deprecated code in Cargo.toml lints settings.

### Added
- Add: example
- Add: new test functions to lib.rs
- Add: whisper_err and whisper_out experimental
- Add: non_exhaustive attribute to IconKind enumeration
- Add: examples folder
- Add: whisper_with_fallback method experimental

### Documentation
- Update: README.md content and structure
- #![doc(html_root_url = "https: //docs.rs/murmur/")]
- Update: lib.rs documentation structure
- Update: `IconKind` enum link in documentation
- Refactor: some crate docs to example .

### Fixed
- Breaking: Change #[non_exhaustive] attribute to WhisperError enum
- Refactor: `Display` trait implementation for `IconKind` to include error handling.

### Changed
- Update: gitignore
- Update: Makefile.toml tasks
- Refactor: from Mutex to RwLock on ICON_MAP in lib.rs file.
- Refactor: from Mutex to RwLock in ICON_MAP in color_map.rs
- Refactor: color map type definitions in color_map.rs
- Refactor: simplifies color map implementation
- Replace: Arc and Mutex with Box in ColorFunction
- Update: changelog
- Update: changelog
- Update: icon_map.rs deprecate NfMd icon variants and replace with NfFa equivalents
- Change: whisper or else fallback ok
- Change: whisper_or_else fallback err
- Rename: to whisper_or_else
- Update: makefile

## [1.2.1] - 2023-12-22

### Changed

- NfFaQuestionCircle: Change color to Red
- NfFaQuestion: Change color to Red

## [1.2.0] - 2023-12-22

### Added

- Add tests icon_map.rs
- NfFaQuestion
- NfFaQuestionCircle
- NfFaTerminal
- NfFaTrash,
- NfFaAngleRight,
- NfFaAngleLeft,
- NfFaAngleUp,
- NfFaAngleDown,
- NfFaThumbsUp,
- NfFaThumbsDown,

## [1.1.0] - 2023-12-22

### Added

New Icons:
- NfMdGreaterThan
- NfMdLessThan
- NfMdEqual
- NfMdThumbsUp
- NfMdThumbsDown
- NfMdFolder
- NfMdFolderOpen

## [1.0.1] - 2023-12-22

### Documentation

- Fix typo in README.md

## [1.0.0] - 2023-12-22

### Added

- Add `messages()`

### Changed

- Update dependencies: owo-colors to 4.0

### Removed

- **Breaking Change**: Remove `message_vec` function from the `Whisper` API. Use the `messages` function with an iterable collection instead.
- Remove dependency: thiserror = "1.0"

### Documentation

- Update docs for the `messages` function.


