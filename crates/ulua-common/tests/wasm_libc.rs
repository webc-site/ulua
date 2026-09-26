//! `wasm_libc` 的 C ABI 面契约（`gmtime_r`/`localtime_r` 的 NULL 回报、
//! `malloc`/`realloc`/`free` 的分配往返）。
//!
//! 集成为 `tests/` 的理由：这些用例只驱动 crate 根的 `pub` 项
//! （`ulua_common::wasm_libc::{gmtime_r, localtime_r, malloc, realloc, free, Tm}`），
//! 无需触碰任何私有实现，按规范 §8 从 `src` 迁入。测私有历法helper
//! `fill_tm`/`days_from_civil` 的 `fill_tm_range_contract_with_c` 依赖内部实现，
//! 保留 `src`（见 `src/wasm_libc.rs` 的 `#[cfg(test)]` 说明）。
//!
//! 门控条件与 `src/wasm_libc.rs` 一致（整体 `target_arch = "wasm32"`；分配器 shim
//! 另在 `target_os = "unknown"` 下提供），故只在 wasm 测试运行器下编译执行，
//! native 门禁不涉及。

#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use core::ptr::null_mut;

use ulua_common::wasm_libc::{Tm, gmtime_r, localtime_r};

fn zero_tm() -> Tm {
  Tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: -1,
    tm_gmtoff: -1,
    tm_zone: null_mut(),
  }
}

#[test]
fn gmtime_r_returns_null_on_overflow() {
  use core::ptr::null;

  let mut tm = zero_tm();
  let huge: i64 = i64::MAX;
  let zero: i64 = 0;
  // Safety: 引用形态实参恒非空、对齐且在调用期间存活，满足 gmtime_r 的
  // 可读 `i64` / 可写 `Tm` 契约；huge 超范围走不写入的 NULL 分支。
  assert!(unsafe { gmtime_r(&huge, &mut tm) }.is_null());
  let ok = unsafe { gmtime_r(&zero, &mut tm) };
  // Safety: 同上契约；ok 非空时指向刚写入的 `tm`，读取存活。
  assert!(!ok.is_null() && unsafe { (*ok).tm_year } == 70);
  // Safety: 显式传 null 验证契约的入参为空回报 NULL 分支，不解引用。
  assert!(unsafe { gmtime_r(null(), &mut tm) }.is_null());
  assert!(unsafe { gmtime_r(&zero, null_mut()) }.is_null());
  assert!(unsafe { localtime_r(&huge, &mut tm) }.is_null());
  assert!(!unsafe { localtime_r(&zero, &mut tm) }.is_null());
}

// 分配器 shim 仅在 `wasm32-unknown-unknown`（无 libc）上由本 crate 提供，
// wasi 目标改用 wasi-libc，故门控与 `malloc`/`realloc`/`free` 的定义处一致。
#[cfg(target_os = "unknown")]
#[test]
fn allocator_roundtrip() {
  use core::ptr::null_mut;

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
