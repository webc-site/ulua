use alloc::borrow::Cow;
use core::{
  ffi::{c_char, c_int, c_void},
  mem::zeroed,
  ptr::addr_of,
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
  // cpp 的文件静态量 gCoverage 只在主线程访问；这里取出 ref 列表的副本，
  // 不把覆盖整个结构（含 Vec）的长生命周期引用跨过 lua_* 调用带出去。
  let l = unsafe { (*addr_of!(G_COVERAGE)).l };
  let function_refs = unsafe { (*addr_of!(G_COVERAGE)).functions.clone() };

  // cpp `fopen(path, "wb")`: 写模式打开, 失败报错返回
  let Ok(file) = File::create(path) else {
    eprintln!("Error opening coverage {path}");
    return;
  };
  let mut out = BufWriter::new(file);

  // cpp 忽略 fprintf 返回值, 此处一致
  let _ = out.write_all(b"TN:\n");

  for &fref in &function_refs {
    unsafe { lua_getref(l, fref) };

    // C++ `LuaDebug ar = {}` — zero-initialized activation record.
    let mut ar: LuaDebug = unsafe { zeroed() };
    unsafe { lua_getinfo(l, -1, c"s".as_ptr(), &mut ar as *mut LuaDebug) };

    // cpp 的 short_src 是内嵌 char[256]，取不到即空串；本端口为裸指针，
    // lua_getinfo 失败时保持 null，必须先判空再构造 CStr。
    let short_src = if ar.short_src.is_null() {
      Cow::Borrowed("")
    } else {
      // SAFETY: 非空的 short_src 由 lua_getinfo("s") 保证为 NUL 结尾字符串
      unsafe { CStr::from_ptr(ar.short_src) }.to_string_lossy()
    };
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
    function_refs.len()
  );
}
