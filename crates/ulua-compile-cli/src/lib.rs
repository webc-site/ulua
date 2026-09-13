extern crate alloc;

pub(crate) mod enums;
pub(crate) mod functions;
pub(crate) mod macros;
pub(crate) mod methods;
pub(crate) mod records;

pub use functions::main::main as run_main;
