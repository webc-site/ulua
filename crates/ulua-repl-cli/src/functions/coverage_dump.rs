use core::{
  ffi::{c_char, c_int, c_void},
  mem::zeroed,
};
use std::{
  ffi::CStr,
  fs::File,
  io::{BufWriter, Write},
};

use ulua_vm::{
  functions::{lua_getcoverage::lua_getcoverage, lua_getinfo::lua_getinfo},
  macros::{lua_getref::lua_getref, lua_pop::lua_pop},
  records::lua_debug::LuaDebug,
};

use crate::functions::{coverage_callback::coverage_callback, coverage_init::G_COVERAGE};

// `extern "C"` adapter so `coverage_callback` can be handed to lua_getcoverage
// as a `LuaCoverage` function pointer; context 携带 `&mut BufWriter<File>`。
unsafe extern "C-unwind" fn coverage_callback_cb(
  context: *mut c_void,
  function: *const c_char,
  linedefined: c_int,
  depth: c_int,
  hits: *const c_int,
  size: usize,
) {
  unsafe {
    let out = &mut *(context.cast::<BufWriter<File>>());
    coverage_callback(out, function, linedefined, depth, hits, size);
  }
}

/// Faithful port of `void coverageDump(const char* path)` (`CLI/src/Coverage.cpp`)
pub fn coverage_dump(path: &str) {
  let coverage = unsafe { &*core::ptr::addr_of!(G_COVERAGE) };
  let l = coverage.l;

  // cpp `fopen(path, "wb")`: 写模式打开, 失败报错返回
  let Ok(file) = File::create(path) else {
    eprintln!("Error opening coverage {path}");
    return;
  };
  let mut out = BufWriter::new(file);

  // cpp 忽略 fprintf 返回值, 此处一致
  let _ = out.write_all(b"TN:\n");

  for &fref in coverage.functions.iter() {
    unsafe { lua_getref(l, fref) };

    // C++ `LuaDebug ar = {}` — zero-initialized activation record.
    let mut ar: LuaDebug = unsafe { zeroed() };
    unsafe { lua_getinfo(l, -1, c"s".as_ptr(), &mut ar as *mut LuaDebug) };

    // SAFETY: lua_getinfo("s") 保证 short_src 指向 NUL 结尾字符串
    let short_src = unsafe { CStr::from_ptr(ar.short_src) }.to_string_lossy();
    let _ = writeln!(out, "SF:{short_src}");

    unsafe {
      lua_getcoverage(
        l,
        -1,
        &mut out as *mut BufWriter<File> as *mut c_void,
        Some(coverage_callback_cb),
      )
    };
    let _ = out.write_all(b"end_of_record\n");

    unsafe { lua_pop(l, 1) };
  }

  // cpp `fclose` 隐式 flush; 失败同样静默
  let _ = out.flush();

  println!(
    "Coverage dump written to {} ({} functions)",
    path,
    coverage.functions.len()
  );
}
