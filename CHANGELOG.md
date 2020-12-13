
# Change Log

Project Changelog. Starts from version 0.2.1

## [0.2.1] - 2020-12-13

Added proper error reporting.

### Added

- json function in GraphQLError for accessing JSON errors
- New tests for error reporting
- Restructured tests crate

### Changed

- Changed 'static lifetime for an endpoint, query parameters in client struct to 'a lifetime
- Changed Debug, Display implementations for GraphQLError to properly display error messages