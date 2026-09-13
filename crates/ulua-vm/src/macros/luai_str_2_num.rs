use core::ffi::c_char;
#[macro_export]
macro_rules! luai_str2num {
  ($s:expr, $p:expr) => {
    $crate::macros::luai_str_2_num::strtod($s, $p)
  };
}

// `strtod` resolves to libc on native, wasi-libc on wasm32-wasip1, and a
// `#[unsafe(no_mangle)]` Rust shim (ulua-common's `strtod_shim`) on wasm32-unknown-
// unknown, which ships no libc. The wasm build previously STUBBED this to
// `{ 0.0 }` and never wrote `endptr`, so `luaO_str2d` dereferenced a null
// `endptr` — `tonumber("1.5")` (and every runtime string→number) crashed /
// returned 0.0 on EVERY wasm build, including the web playground. Native was
// never affected (real libc).
unsafe extern "C" {
  pub fn strtod(s: *const c_char, endptr: *mut *mut c_char) -> f64;
}

pub use luai_str2num;
