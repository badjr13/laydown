# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Prompt user if they're attempting to overwrite an existing archive file

## [1.5.0] - 2022-07-22
### Added
- Automatically clear out existing Standup after `archive` command is run

## [1.4.1] - 2022-07-18
### Fixed
-  Fix bug where data file is empty upon initialization

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

[Unreleased]: https://github.com/badjr13/laydown
<!-- Obtained by going to last commit before version bump and `Browse Files` -->
[1.4.1]: https://github.com/badjr13/laydown/tree/69b8a22901e3cb639133282b80d7f4c4b19a05c5
[1.4.0]: https://github.com/badjr13/laydown/tree/0bb487815550b9182398508c22b8134b30844724
[1.3.0]: https://github.com/badjr13/laydown/tree/f6b23dcdd5b86796831e8e7f81282bb8341aad91
[1.2.0]: https://github.com/badjr13/laydown/tree/5a10fe65baac320d6a557a66af6372f690c04118
[1.1.0]: https://github.com/badjr13/laydown/tree/5b8c4a82a2362e0ed6a902e8166bb570f6dab403
[1.0.0]: https://github.com/badjr13/laydown/tree/951eb7d67472ca09c93dc22cb65541f71a8e23e9
