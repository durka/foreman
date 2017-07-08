//! `foreman`: Rust build script assistant
//!
//! "Don't pick up those environment variables yourself.
//!  Just tell the foreman what to do with the cargo."
//!
//! This crate contains utilities for build scripts to talk to Cargo, abstracting away the input
//! (via special environment variables that Cargo sets) and output (via special patterns that the
//! build script prints to stdout).

#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate strum_macros;
extern crate strum;
extern crate walkdir;

use std::{env, num, str};

#[macro_use]
mod macros;

error_chain! {
    foreign_links {
        Env(env::VarError)
        /// A required environment variable did not exist or was not valid UTF-8
        ;

        ParseInt(num::ParseIntError)
        /// An environment variable could not be parsed as an integer
        ;

        ParseBool(str::ParseBoolError)
        /// An environment variable could not be parsed as a boolean
        ;

        ParseProfile(strum::ParseError)
        /// An environment variable could not be parsed as a compilation profile
        ;
    }
}

mod types;
pub use types::*;

mod input;
pub use input::*;

mod output;
pub use output::*;
