//! # CastleCore
//!
//! It's my crate!

// Public constants
pub(crate) const CC_VER: &str = "v0.0.4";

/// A temporary module for debugging functions.
pub mod functions;

/// Return current CastleCore version
pub fn cc_ver() -> String {
    (&CC_VER).to_string()
}
