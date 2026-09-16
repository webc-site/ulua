extern crate alloc;

pub mod functions;
pub mod methods;
pub mod records;

mod util;

#[cfg(feature = "wasm")]
pub mod wasm;
