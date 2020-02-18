# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

## [v0.0.8] - 2020-02-20

- INPUTMUX is back

## [v0.0.7] - 2020-02-16

- Actually build the new patched SVD...

## [v0.0.6] - 2020-02-20

- Update SVD to version from SDK 2.7.0
- Use svd2rust 0.17.0 (changes most files somewhat)
- Use svdtools 0.1.0
- Bump dependencies
- ADC and PUF fixes

## [v0.0.5] - 2019-09-19

### Changed

- Update SVD to version from SDK 2.6.3, this is breaking
  change that fixes a bunch of things
- Revert `UTICK0` rename from v0.0.3
- Update dependencies

### Added
- New SVD files from SDK 2.6.3

### Removed
- Unnecessary documentation generation
- Old SDK 2.5.1 files
- Unnecessary SDK 2.6.2 files

## [v0.0.4] - 2019-08-31

### Changed

- Delete `SYSCON.PRESECTRLX?`, as `svd2rust` does not
  handle them properly

## [v0.0.3] - 2019-08-31

### Changed

- Rename `UTICK0` to `UTICK` like UM

## [v0.0.2] - 2019-08-23

### Added

- CI, via CircleCI
- Better README

### Changed

- Delete `SYSCON.AHBCLKCTRLX?`, as `svd2rust` does not
  handle them properly

## [v0.0.1] - 2019-08-18

### Added

- Initial release
- Delete `INPUTMUX` register to appease `svd2rust`.

