#[cfg(not(target_arch = "wasm32"))]
pub use ulua_common::macros::win_32_lean_and_mean::WIN32_LEAN_AND_MEAN;

#[cfg(not(target_arch = "wasm32"))]
pub const WIN_32_LEAN_AND_MEAN: () = ();
