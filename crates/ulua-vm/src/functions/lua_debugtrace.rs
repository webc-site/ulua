//! Source: `VM/src/ldebug.cpp:630-677` (hand-ported)

use core::{cell::UnsafeCell, ffi::c_char, mem::zeroed, ptr::copy_nonoverlapping};

use itoa::Buffer;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{append::append, lua_getinfo::lua_getinfo},
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

const BUF_LEN: usize = 4096;

// cpp `ldebug.cpp:632` 的函数级 `static char buf[4096]` 在 Rust 下若照抄为
// `static mut`，两线程各持 VM 并发调用即数据竞争 UB；改为线程局部缓冲，
// 线程内语义（返回指针在下一次本线程调用前有效）与上游一致，且彻底消除
// 跨线程竞争。FFI 返回形态（`*const c_char`）保持不变。
thread_local! {
  static TRACE_BUF: UnsafeCell<[c_char; BUF_LEN]> = const { UnsafeCell::new([0; BUF_LEN]) };
}

/// Write the concatenation of `parts` into `buf` as a NUL-terminated C string,
/// truncating if needed (the pure-Rust stand-in for the original `snprintf`
/// calls, which have no symbol to bind on `wasm32-unknown-unknown` and trapped
/// in the browser). 数字段由 `itoa` 格式化后作为 part 传入，避免每帧 `format!`
/// 堆分配。
///
/// # Safety
///
/// `buf` 必须非空（`len >= 1`），否则 `buf.len() - 1 - n` 会在无符号减法下溢出；
/// 输出以 NUL 结尾且总长不超过 `buf.len()`。
unsafe fn write_c_str(buf: &mut [c_char], parts: &[&str]) {
  // Safety: 契约保证 `l` 调用栈自 level 起逐级可读、空帧即止，回溯信息经回调透出且不改栈
  unsafe {
    let mut n = 0;
    for part in parts {
      let take = part.len().min(buf.len() - 1 - n);
      copy_nonoverlapping(
        part.as_ptr() as *const c_char,
        buf.as_mut_ptr().add(n),
        take,
      );
      n += take;
    }
    buf[n] = 0;
  }
}

/// # Safety
/// `l` 须存活且 `ci`/`base_ci` 指向同一 CallInfo 数组（`offset_from` 前提），回溯期间栈不被重排
/// （`lua_getinfo` 不触发 GC/扩容）。返回指针指向线程局部缓冲，仅在本线程下一次调用前有效。
/// cpp ldebug.cpp:668。
pub unsafe fn lua_debugtrace(l: *mut LuaState) -> *const c_char {
  unsafe {
    const LIMIT1: i32 = 10;
    const LIMIT2: i32 = 10;

    TRACE_BUF.with(|cell| {
      // 线程局部缓冲在本线程再次调用前稳定有效，指针可安全返回给 FFI 调用方。
      let buf_ptr: *mut c_char = cell.get().cast();

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
          let mut num = Buffer::new();
          write_c_str(&mut line, &[":", num.format(ar.currentline)]);

          offset = append(buf_ptr, BUF_LEN, offset, line.as_ptr());
        }

        if !ar.name.is_null() {
          offset = append(buf_ptr, BUF_LEN, offset, c" function ".as_ptr());
          offset = append(buf_ptr, BUF_LEN, offset, ar.name);
        }

        offset = append(buf_ptr, BUF_LEN, offset, c"\n".as_ptr());

        if depth > LIMIT1 + LIMIT2 && level == LIMIT1 - 1 {
          let mut skip: [c_char; 32] = [0; 32];
          let mut num = Buffer::new();
          write_c_str(
            &mut skip,
            &["... (+", num.format(depth - LIMIT1 - LIMIT2), " frames)\n"],
          );

          offset = append(buf_ptr, BUF_LEN, offset, skip.as_ptr());

          level = depth - LIMIT2 - 1;
        }

        level += 1;
      }

      LUAU_ASSERT!(offset < BUF_LEN);
      *buf_ptr.add(offset) = 0;

      buf_ptr as *const c_char
    })
  }
}
