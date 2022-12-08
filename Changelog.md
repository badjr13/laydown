# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

**empty**

## [2.6.4] - 2022-12-08

## Added

- Install instructions to README

## [2.6.3] - 2022-12-06

## Fixed

- Add file extension to Release binaries

## [2.6.2] - 2022-12-06

## Fixed

- Replace "program-name" with "Laydown" in Release action

## [2.6.1] - 2022-12-06

## Fixed

- Release action for tag pushing was missing "v"

## [2.6.0] - 2022-12-03

## Added

- Generate binaries and upload to release when tag is created

## [2.5.0] - 2022-11-25

## Added

- Unit tests for data_file
- Linting and Formatting to GitHub Actions

## Fixed

- Small syntax update based on 'Clippy' recommendation

## [2.4.1] - 2022-11-22

## Fixed

- Issue with EDIT feature Pull Request

## [2.4.0] - 2022-11-22

## Fixed
- Repair EDIT functionality which broke during migration to Clap

## Changed
- Clean up data_file.rs in preparation for unit testing

## [2.3.0] - 2022-11-13

## Added

- Added numerous integration tests
- Added github workflow for automated testing on push to main
- Implement default for Standup

## Changed

- Dont write out empty line after Sidebar(last) section of Standup

## [2.2.0] - 2022-10-26

## Changed

- Update help message

## [2.1.0] - 2022-10-26

### Changed

- Switch main arguments to be Clap subcommands

## [2.0.1] - 2022-10-18

### Fixed

- Typos in help message

## [2.0.0] - 2022-10-14

### Changed

- Updated CLI application from Rust standard library to Clap.rs

### Removed

- Tests. I know this is bad. I want to rewrite them.

## [1.6.0] - 2022-08-16

### Added

- Only deserialize ron data file when needed

### Fixed

- Fix archiving bug

## [1.5.1] - 2022-08-13

### Added

- Fix missing history bug
- Prompt user if they're attempting to overwrite an existing archive file

## [1.5.0] - 2022-07-22

### Added

- Automatically clear out existing Standup after `archive` command is run

## [1.4.1] - 2022-07-18

### Fixed

- Fix bug where data file is empty upon initialization

## [1.4.0] - 2022-05-27

### Added

- Add ability to print out location of config directory
- Create view for empty sections
- Add ability to create multiple items at once

## [1.3.0] - 2022-05-18

### Added

- Add ability to Archive Daily Standup
- Implement Display on Standup struct

## [1.2.0] - 2022-05-04

### Added

- Add ability to `undo`

### Fixed

- App would crash if `EDITOR` environment variable is not set

## [1.1.0] - 2022-04-26

### Added

- Add `--help` command
- Add support for `EDITOR` environment variable for use with `edit` command

## [1.0.0] - 2022-04-13

### Added

- Did, Doing, Blockers, Sidebars, Clear, Edit, Help

[unreleased]: https://github.com/badjr13/laydown

<!-- Obtained by going to last commit before version bump and `Browse Files` -->
[2.6.4]: https://github.com/badjr13/laydown/tree/3d695b989884dfdb023af8b4cb481b0f68692494
[2.6.3]: https://github.com/badjr13/laydown/tree/891348e60bbbe7c8a0b9c4a43dd239c93549f7ab
[2.6.2]: https://github.com/badjr13/laydown/tree/7d10f90a58acc958b7fe75da9ec1bdf6579f405b
[2.6.1]: https://github.com/badjr13/laydown/tree/bcdade7de8d9643c576b5c39a0a1f2dbde456640
[2.6.0]: https://github.com/badjr13/laydown/tree/46db895dc16a86d9805df36889fab89e08e16c95
[2.5.0]: https://github.com/badjr13/laydown/tree/3481d6c4d3b21f83b5237a325df8416c07a0b26d
[2.4.1]: https://github.com/badjr13/laydown/tree/65adc66a3233f7db9a8fd8311a534252388cdd35
[2.4.0]: https://github.com/badjr13/laydown/tree/da09128b8ec09a3c4cc9357cac1d1b780b654073
[2.3.0]: https://github.com/badjr13/laydown/tree/246cb54ff78ca9b185435c464ce6bc59abea4b82
[2.2.0]: https://github.com/badjr13/laydown/tree/31ca0f2b0c98c506b6bc333b4aaf7478516c2d7a
[2.1.0]: https://github.com/badjr13/laydown/tree/64661fd0ad56006ffbd1e68f379de9a8827673d4
[2.0.1]: https://github.com/badjr13/laydown/tree/dcbde0fa55d97e4e8d2a6d2344ea8201b38de267
[2.0.0]: https://github.com/badjr13/laydown/tree/1d3239c132c39ec3f2a44dfd837f53f6d7e54e87
[1.6.0]: https://github.com/badjr13/laydown/tree/00fcf6c1385152e5aa2d3a359482e86af194494a
[1.5.1]: https://github.com/badjr13/laydown/tree/f246b3a9e1d85376967a23b4a7e2c93e1cac81e0
[1.5.0]: https://github.com/badjr13/laydown/tree/a35ca80390b74b61dcd4771119da74b918b476d7
[1.4.1]: https://github.com/badjr13/laydown/tree/69b8a22901e3cb639133282b80d7f4c4b19a05c5
[1.4.0]: https://github.com/badjr13/laydown/tree/0bb487815550b9182398508c22b8134b30844724
[1.3.0]: https://github.com/badjr13/laydown/tree/f6b23dcdd5b86796831e8e7f81282bb8341aad91
[1.2.0]: https://github.com/badjr13/laydown/tree/5a10fe65baac320d6a557a66af6372f690c04118
[1.1.0]: https://github.com/badjr13/laydown/tree/5b8c4a82a2362e0ed6a902e8166bb570f6dab403
[1.0.0]: https://github.com/badjr13/laydown/tree/951eb7d67472ca09c93dc22cb65541f71a8e23e9
