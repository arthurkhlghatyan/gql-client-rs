
# Change Log

Project Changelog. Starts from version 0.2.1

## [1.0.1] - 2021-13-04

Minor bug fix.

### Changed

- Fixed failing compilation with `serde::export` ([change occured in this commit](https://github.com/serde-rs/serde/commit/dd1f4b483ee204d58465064f6e5bf5a457543b54))

## [1.0.0] - 2021-06-01

Release V1

### Added

- Configured CI to ensure WebAssembly support
- Added missing path property GraphQLErrorMessage struct
- Upgrade tokio to v1.0 in dev dependencies
- Upgrade reqwest to v0.11

### Changed

- Made location, extensions fields optional in GraphQLErrorMessage struct

## [0.2.1] - 2020-12-13

Added proper error reporting.

### Added

- json function in GraphQLError for accessing JSON errors
- New tests for error reporting
- Restructured tests crate

### Changed

- Changed 'static lifetime for an endpoint, query parameters in client struct to 'a lifetime
- Changed Debug, Display implementations for GraphQLError to properly display error messages