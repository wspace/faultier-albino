extern crate getopts;
extern crate log;

pub static VERSION_MAJOR: uint = 0;
pub static VERSION_MINOR: uint = 1;
pub static VERSION_TINY: uint = 0;
pub static PRE_RELEASE: bool = true;

pub fn version() -> String {
    format!(
        "{}.{}.{}{}",
        VERSION_MAJOR,
        VERSION_MINOR,
        VERSION_TINY,
        if PRE_RELEASE { "-pre" } else { "" }
    )
}

pub mod command;
pub mod util;
