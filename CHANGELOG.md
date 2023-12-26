# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [[Unreleased]] - 2023-12-25

## [2.0.0] - 2023-12-26

### Breaking Changes
- Breaking Change: NfMd Variants are deprecated and will be replaced by their equivalent NfFa Variant in the next release
- Breaking Change: #[non_exhaustive] attribute to WhisperError enum
- Breaking Change: non_exhaustive attribute to IconKind enumeration
- Breaking Change: Remove all md variants from icon_map.rs

### Changed
- Refactor: from Mutex to RwLock on ICON_MAP .

### Added
- Add: whisper_err and whisper_out experimental
- Add: examples folder

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


