pub const VERSION_MAJOR: usize = 0;
pub const VERSION_MINOR: usize = 2;
pub const VERSION_TINY: usize = 0;
pub const PRE_RELEASE: bool = false;

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
