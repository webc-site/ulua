//! `wasm_libc` 的 C ABI 面契约（`malloc`/`realloc`/`free` 的分配往返）。
//!
//! 原文件另有 `gmtime_r`/`localtime_r` 的 NULL 回报契约：时间分解已迁为
//! `ulua-vm` 内的纯 Rust 实现（`jiff`），wasm shim 删除，该契约随之撤销。
//!
//! 门控条件与 `src/wasm_libc.rs` 一致（整体 `target_arch = "wasm32"`；分配器 shim
//! 另在 `target_os = "unknown"` 下提供），故只在 wasm 测试运行器下编译执行，
//! native 门禁不涉及。

#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#![allow(dead_code)] // 仅剩 allocator_roundtrip，其门控见下

use core::ptr::null_mut;

// 分配器 shim 仅在 `wasm32-unknown-unknown`（无 libc）上由本 crate 提供，
// wasi 目标改用 wasi-libc，故门控与 `malloc`/`realloc`/`free` 的定义处一致。
#[cfg(target_os = "unknown")]
#[test]
fn allocator_roundtrip() {
  use ulua_common::wasm_libc::{free, malloc, realloc};

  // Safety: malloc(8) 契约上返回非空 8 字节块，u64 写入落在块内。
  let p = unsafe { malloc(8) };
  assert!(!p.is_null());
  unsafe { (p as *mut u64).write(0xDEAD_BEEF_CAFE_F00D) };
  // Safety: p 是本模块分配且未释放的指针，realloc 后原 p 失效、内容前 8 字节
  // 保留进新块，读取落在新块用户区内。
  let q = unsafe { realloc(p, 64) };
  assert!(!q.is_null());
  assert_eq!(unsafe { *(q as *mut u64) }, 0xDEAD_BEEF_CAFE_F00D);
  unsafe { free(q) };
  // size+HEADER 溢出回绕防御：malloc/realloc 均回报 NULL（NULL/超界入参
  // 本就不解引用，Safety 由契约覆盖）。
  assert!(unsafe { malloc(usize::MAX) }.is_null());
  assert!(unsafe { realloc(null_mut(), usize::MAX) }.is_null());
  // malloc(0) 给可释放唯一指针；free(NULL) 为 no-op。
  let z = unsafe { malloc(0) };
  assert!(!z.is_null() && z != unsafe { malloc(0) });
  unsafe { free(z) };
  unsafe { free(null_mut()) };
}
