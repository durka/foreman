//! `foreman`: Rust build script assistant
//!
//! "Don't pick up those environment variables yourself.
//!  Just tell the foreman what to do with the cargo."
//!
//! This crate contains utilities for build scripts to talk to Cargo, abstracting away the input
//! (via special environment variables that Cargo sets) and output (via special patterns that the
//! build script prints to stdout).

#![deny(missing_docs)]

#[macro_use] extern crate derive_fail;
#[macro_use] extern crate strum_macros;
extern crate failure;
extern crate strum;
extern crate walkdir;

use failure::Context;

#[macro_use] mod macros;

/// Errors that can be produced by fallible functions
#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// Env var missing or not UTF-8
    #[fail(display = "{}", _0)]
    Env(&'static str),

    /// Could not parse int
    #[fail(display = "{} is not an integer", _0)]
    ParseInt(String),

    /// Could not parse bool
    #[fail(display = "{} is not a boolean", _0)]
    ParseBool(String),

    /// Could not parse compilation profile (see [Profile](enum.Profile.html))
    #[fail(display = "{} is not a boolean", _0)]
    ParseProfile(String),
}

/// Result alias for fallible functions
pub type Result<T> = std::result::Result<T, Context<ErrorKind>>;

mod types;
pub use types::*;

mod input;
pub use input::*;

mod output;
pub use output::*;
