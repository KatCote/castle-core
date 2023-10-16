//! # CastleCore
//!
//! Core Engine for development (Altenstein and same projects.)
//! Will pack with map (and same) editor base on CastleCore.

// Public constants
pub(crate) const CC_VER: &str = "v0.0.7";

// Heavy lines for write borders
pub(crate) const LU_CORNER: char = '┏';
pub(crate) const LD_CORNER: char = '┗';
pub(crate) const RU_CORNER: char = '┓';
pub(crate) const RD_CORNER: char = '┛';
pub(crate) const UD_LINE: char = '━';
pub(crate) const LR_LINE: char = '┃';
pub(crate) const UD_T: char = '┳';
pub(crate) const DU_T: char = '┻';
//pub(crate) const LR_T: char = '┣';
//pub(crate) const RL_T: char = '┫';
//pub(crate) const CROSS: char = '╋';

// Double pack: ║ ═ ╔ ╗ ╚ ╝ ╠ ╣ ╦ ╩ ╬ 

/// A temporary module for debugging functions.
pub mod functions;
/// A module containing all the functions related to the borders (temporary description)
pub mod window;
/// A module containing all the functions related to the screen (temporary description)
pub mod screen;
/// A module containing all the functions related to the render (temporary description)
pub mod render;

/// Return current CastleCore version
pub fn cc_ver() -> &'static str {
    &CC_VER
}
