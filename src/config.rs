//! Configurable properties for brute forcing.
//!
//! All these constants must be set to correct values for your brute force attempt, before you
//! start using the tool.

/// The dots we should generate patterns on for brute forcing.
///
/// The indices are row-by-row based, for example:
///
/// ```
/// 0 1 2    00 01 02 03    00 01 02 03 04
/// 3 4 5    04 05 06 07    05 06 07 08 09
/// 6 7 8    08 09 10 11    10 11 12 13 14
///          12 13 14 15    15 16 17 18 19
///                         20 21 22 23 24
/// ```
pub const DOTS: [u16; 12] = [1, 2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15];

/// The size of the pattern grid, probably 3, 4 or 5.
pub const GRID_SIZE: u16 = 4;

/// The minimum length of patterns to attempt.
pub const PATTERN_LEN_MIN: u16 = 4;

/// The maximum length of patterns to attempt.
pub const PATTERN_LEN_MAX: u16 = 6;

/// The maximum distance between dots in a pattern.
///
/// Imagine the following distances for each dot, where `X` is the center:
/// ```
/// 2 2 2 2 2
/// 2 1 1 1 2
/// 2 1 X 1 2
/// 2 1 1 1 2
/// 2 2 2 2 2
/// ```
pub const PATTERN_DISTANCE_MAX: u16 = 1;