# Changelog
All notable changes to this project will be documented in this file.

## [0.1.7] - 28/02/2021 (Strings & Bug fixing session)
### Added
- Changelog
- Updated dependencies
- Allowed blank lines in compiler
- Strings
- New ImpossibleOperation error for undefined operations
- Added in string concatenation

### Fixed
- Fixed mixed up column & line values
- Fixed compiler panics on comments
- Fixed weird EOI panics
- Fixed incorrect length & start positions in lexer

## [0.1.6] - 27/02/2021 (Comparison and Equality)
### Added
- Correct handling of literals
- Added in comparison operators
- Added in equality operators
- Errors now unwind expressions into their component parts for better debugging

### Removed
- Quotes from value displaying for booleans and nil

## [0.1.5] - 26/02/2021 (Error overhaul)
### Added
- Allowed for unicode characters in errors
- Improved error column and line detection
- Added in easy error debugging display

### Fixed
- Fixed VM stack overcrowding bug

### Removed
- Removed `Error:` marker from error messages

## [0.1.4] - 26/02/2021 (Colours, booleans, nils and comments)
### Added
- Used the `lliw` crate to add colours
- Booleans
- Nil value
- New MismatchedTypes error
- Added in multiline and singleline comments

## [0.1.3] - 24/02/2021 (Even more operators)
### Added
- Power operator

## [0.1.2] - 24/02/2021 (Clean up and more operators)
### Added
- Modulo operator

### Removed
- Bloated swap files

## [0.1.1] - 24/02/2021 (Improved user interaction)
### Added
- Improved ExpectedToken error message
- Added welcome message to REPL
- Changed how the REPL looks

## [0.1.0] - 24/02/2021 (Initial commit)
### Added
- Basic calculator bytecode VM
