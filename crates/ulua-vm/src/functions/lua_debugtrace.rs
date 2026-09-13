//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:630:lua_debugtrace`
//! Source: `VM/src/ldebug.cpp:630-677` (hand-ported)

use core::{ffi::c_char, mem::zeroed, ptr::copy_nonoverlapping};

use crate::{
  functions::{append::append, lua_getinfo::lua_getinfo},
  records::lua_debug::LuaDebug,
  type_aliases::lua_state::lua_State,
};

static mut BUF: [c_char; 4096] = [0; 4096];

/// Write `s` into `buf` as a NUL-terminated C string, truncating if needed
/// (the pure-Rust stand-in for the original `snprintf` calls, which have no
/// symbol to bind on `wasm32-unknown-unknown` and trapped in the browser).
unsafe fn write_c_str(buf: &mut [c_char], s: &str) {
  unsafe {
    let n = s.len().min(buf.len() - 1);
    copy_nonoverlapping(s.as_ptr() as *const c_char, buf.as_mut_ptr(), n);
    buf[n] = 0;
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_debugtrace(l: *mut lua_State) -> *const c_char {
  unsafe {
    const LIMIT1: i32 = 10;
    const LIMIT2: i32 = 10;
    const BUF_LEN: usize = 4096;
    let buf_ptr = &raw mut BUF as *mut c_char;

    let depth: i32 = (*l).ci.offset_from((*l).base_ci) as i32;
    let mut offset: usize = 0;

    let mut ar: LuaDebug = zeroed();

    let mut level: i32 = 0;
    while lua_getinfo(l, level, c"sln".as_ptr(), &mut ar as *mut LuaDebug) != 0 {
      if !ar.short_src.is_null() {
        offset = append(buf_ptr, BUF_LEN, offset, ar.short_src);
      }

      if ar.currentline > 0 {
        let mut line: [c_char; 32] = [0; 32];
        write_c_str(&mut line, &alloc::format!(":{}", ar.currentline));

        offset = append(buf_ptr, BUF_LEN, offset, line.as_ptr());
      }

      if !ar.name.is_null() {
        offset = append(buf_ptr, BUF_LEN, offset, c" function ".as_ptr());
        offset = append(buf_ptr, BUF_LEN, offset, ar.name);
      }

      offset = append(buf_ptr, BUF_LEN, offset, c"\n".as_ptr());

      if depth > LIMIT1 + LIMIT2 && level == LIMIT1 - 1 {
        let mut skip: [c_char; 32] = [0; 32];
        write_c_str(
          &mut skip,
          &alloc::format!("... (+{} frames)\n", depth - LIMIT1 - LIMIT2),
        );

        offset = append(buf_ptr, BUF_LEN, offset, skip.as_ptr());

        level = depth - LIMIT2 - 1;
      }

      level += 1;
    }

    ulua_common::macros::luau_assert::LUAU_ASSERT!(offset < BUF_LEN);
    *buf_ptr.add(offset) = 0;

    buf_ptr as *const c_char
  }
}
