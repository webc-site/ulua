// 边界契约测试：null/裸指针系真实 wasm/libc FFI 边界（既有约定 review.md §2）
//! `wasm_libc` 的 C ABI 面契约（`malloc`/`realloc`/`free` 的分配往返）。
//!
//! 原文件另有 `gmtime_r`/`localtime_r` 的 NULL 回报契约：时间分解已迁为
//! `ulua-vm` 内的纯 Rust 实现（`jiff`），wasm shim 删除，该契约随之撤销。
//!
//! 门控条件与 `src/wasm_libc.rs` 一致：定义处与本合同用例均为**同一枚**
//! 组合门 `all(target_arch = "wasm32", target_os = "unknown")`（模块级/文件级
//! 单重 `#![cfg]`，wasi 目标由 wasi-libc 提供这些符号、整模块门出），非两条
//! 独立事实，故只在 wasm 测试运行器下编译执行，native 门禁不涉及。

#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use core::ptr::null_mut;

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
