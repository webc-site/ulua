use core::{
  ffi::{c_char, c_int, c_void},
  mem::zeroed,
};

use ulua_vm::{
  functions::{lua_getcoverage::lua_getcoverage, lua_getinfo::lua_getinfo},
  macros::{lua_getref::lua_getref, lua_pop::lua_pop},
  records::lua_debug::LuaDebug,
};

use crate::functions::{coverage_callback::coverage_callback, coverage_init::G_COVERAGE};

unsafe extern "C" {
  fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
  fn fclose(stream: *mut c_void) -> c_int;
  fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
  fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
}

// `extern "C"` adapter so `coverage_callback` can be handed to lua_getcoverage
// as a `LuaCoverage` function pointer.
unsafe extern "C-unwind" fn coverage_callback_cb(
  context: *mut c_void,
  function: *const c_char,
  linedefined: c_int,
  depth: c_int,
  hits: *const c_int,
  size: usize,
) {
  unsafe {
    coverage_callback(context, function, linedefined, depth, hits, size);
  }
}

// Faithful port of `void coverageDump(const char* path)`.
pub fn coverage_dump(path: &str) {
  unsafe {
    let coverage = &*core::ptr::addr_of!(G_COVERAGE);
    let l = coverage.l;

    let path_c = alloc::format!("{}\0", path);
    let f = fopen(path_c.as_ptr() as *const c_char, c"wb".as_ptr());
    if f.is_null() {
      eprintln!("Error opening coverage {}", path);
      return;
    }

    fputs(c"TN:\n".as_ptr(), f);

    for &fref in coverage.functions.iter() {
      lua_getref(l, fref);

      // C++ `LuaDebug ar = {}` — zero-initialized activation record.
      let mut ar: LuaDebug = zeroed();
      lua_getinfo(l, -1, c"s".as_ptr(), &mut ar as *mut LuaDebug);

      fprintf(f, c"SF:%s\n".as_ptr(), ar.short_src);
      lua_getcoverage(l, -1, f, Some(coverage_callback_cb));
      fputs(c"end_of_record\n".as_ptr(), f);

      lua_pop(l, 1);
    }

    fclose(f);

    println!(
      "Coverage dump written to {} ({} functions)",
      path,
      coverage.functions.len()
    );
  }
}
