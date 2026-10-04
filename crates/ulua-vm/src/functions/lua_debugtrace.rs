//! Source: `VM/src/ldebug.cpp:630-677` (hand-ported)

use core::{cell::UnsafeCell, slice::from_raw_parts};

use itoa::Buffer;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{append::append_bytes, lua_getinfo::lua_getinfo},
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

const BUF_LEN: usize = 4096;

// cpp `ldebug.cpp:632` 的函数级 `static char buf[4096]` 在 Rust 下若照抄为
// `static mut`，两线程各持 VM 并发调用即数据竞争 UB；改为线程局部缓冲，
// 线程内语义（返回窗在下一次本线程调用前有效）与上游一致，且彻底消除
// 跨线程竞争。review.md §10：返回形态收为 `&'a [u8]`（旧 `*const c_char` 的
// NUL 终止位仍写入，供 cpp 观察面等值；Rust 消费面按长度读取）。
thread_local! {
  static TRACE_BUF: UnsafeCell<[u8; BUF_LEN]> = const { UnsafeCell::new([0; BUF_LEN]) };
}

/// # Safety
/// `l` 须存活且 `ci`/`base_ci` 指向同一 CallInfo 数组（`offset_from` 前提），回溯期间栈不被重排
/// （`lua_getinfo` 不触发 GC/扩容）。返回窗指向线程局部缓冲，仅在本线程下一次调用前有效
/// （`'a` unconstrained 系该既有寿命契约的类型表达，与原裸指针返回同级）。
/// cpp ldebug.cpp:668。
pub unsafe fn lua_debugtrace<'a>(l: *mut LuaState) -> &'a [u8] {
  unsafe {
    const LIMIT1: i32 = 10;
    const LIMIT2: i32 = 10;

    TRACE_BUF.with(|cell| {
      // 线程局部缓冲在本线程再次调用前稳定有效，切片窗可安全返回给调用方。
      let buf = &mut *cell.get();

      let depth: i32 = (*l).ci.offset_from((*l).base_ci) as i32;
      let mut offset: usize = 0;

      let mut ar = LuaDebug::default();
      let mut num = Buffer::new();

      let mut level: i32 = 0;
      while lua_getinfo(l, level, b"sln", &mut ar) != 0 {
        if !ar.short_src.bytes().is_empty() {
          offset = append_bytes(buf, offset, ar.short_src.bytes());
        }

        if ar.currentline > 0 {
          offset = append_bytes(buf, offset, b":");
          offset = append_bytes(buf, offset, num.format(ar.currentline).as_bytes());
        }

        if let Some(name) = ar.name {
          offset = append_bytes(buf, offset, b" function ");
          offset = append_bytes(buf, offset, name);
        }

        offset = append_bytes(buf, offset, b"\n");

        if depth > LIMIT1 + LIMIT2 && level == LIMIT1 - 1 {
          offset = append_bytes(buf, offset, b"... (+");
          offset = append_bytes(buf, offset, num.format(depth - LIMIT1 - LIMIT2).as_bytes());
          offset = append_bytes(buf, offset, b" frames)\n");

          level = depth - LIMIT2 - 1;
        }

        level += 1;
      }

      LUAU_ASSERT!(offset < BUF_LEN);
      buf[offset] = 0;

      // 线程局部窗借出：`buf` 由 TLS 持有、本线程再次调用前稳定，`'a` 系该
      // 既有寿命契约的类型表达（与原裸指针返回同级）
      from_raw_parts(buf.as_ptr(), offset)
    })
  }
}
