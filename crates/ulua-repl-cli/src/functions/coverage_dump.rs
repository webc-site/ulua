use alloc::borrow::Cow;
use core::{
  ffi::{c_char, c_int, c_void},
  mem::zeroed,
  ptr::addr_of_mut,
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
  // cpp 的文件静态量 gCoverage 只在主线程（coverageDump）访问，无需同步。
  // 与 counters_dump 一致：循环体只读 functions、回调只写 out（BufWriter），
  // 字段不相交，直接经裸指针原地遍历，免掉整表 clone。
  let coverage_ptr = addr_of_mut!(G_COVERAGE);
  // SAFETY: 单线程 REPL 路径，G_COVERAGE 只在此处与 coverage_init/coverage_track
  // 串行访问，取地址后即读 l 字段。
  let l = unsafe { (*coverage_ptr).l };

  // cpp `fopen(path, "wb")`: 写模式打开, 失败报错返回
  let Ok(file) = File::create(path) else {
    eprintln!("Error opening coverage {path}");
    return;
  };
  let mut out = BufWriter::new(file);

  // cpp 忽略 fprintf 返回值, 此处一致
  let _ = out.write_all(b"TN:\n");

  // SAFETY: 见上；`functions` 在遍历期间不被回调改动。
  for &fref in unsafe { (*coverage_ptr).functions.iter() } {
    // SAFETY: l 为 coverage_init 记录的 VM 主线程，fref 是已注册的表引用。
    unsafe { lua_getref(l, fref) };

    // C++ `LuaDebug ar = {}` — zero-initialized activation record.
    let mut ar: LuaDebug = unsafe { zeroed() };
    // SAFETY: 栈顶是 lua_getref 压入的函数，ar 为可写且零初始化的 LuaDebug。
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

    // SAFETY: out 在本作用域内存活，context 与回调签名匹配（只写该 BufWriter）。
    unsafe {
      lua_getcoverage(
        l,
        -1,
        &mut out as *mut BufWriter<File> as *mut c_void,
        Some(coverage_callback_cb),
      )
    };
    let _ = out.write_all(b"end_of_record\n");

    // SAFETY: 与 lua_getref 配平，弹出栈顶函数。
    unsafe { lua_pop(l, 1) };
  }

  // cpp `fclose` 隐式 flush; 失败同样静默
  let _ = out.flush();

  // SAFETY: 见函数开头的单线程访问前提。
  let dumped = unsafe { (*coverage_ptr).functions.len() };
  println!("Coverage dump written to {path} ({dumped} functions)");
}
