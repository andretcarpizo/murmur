# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

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


