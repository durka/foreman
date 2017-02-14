#[macro_use] extern crate error_chain;
#[macro_use] extern crate strum_macros;
extern crate strum;

use std::{env, num, str};

#[macro_use] mod macros;

error_chain! {
    foreign_links {
        Env(env::VarError);
        ParseInt(num::ParseIntError);
        ParseBool(str::ParseBoolError);
        ParseProfile(strum::ParseError);
    }
}

mod types;
pub use types::*;

mod input;
pub use input::*;

mod output;
pub use output::*;

