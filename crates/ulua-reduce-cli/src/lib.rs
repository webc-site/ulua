extern crate alloc;

pub(crate) mod enums;
pub(crate) mod functions;
pub(crate) mod methods;
pub(crate) mod records;
pub(crate) mod type_aliases;

pub use functions::main::run as run_main;
