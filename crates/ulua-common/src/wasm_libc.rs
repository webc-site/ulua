//! Minimal libc surface for the `wasm32-unknown-unknown` target.
//!
//! `wasm32-unknown-unknown` ships no libc, so the handful of C functions the
//! faithful translation declares via `extern "C"` (the VM's C-style allocator,
//! `string.format`'s `snprintf`, etc.) have no
//! symbol to bind to and would otherwise surface as unresolved `env` imports in
//! the generated wasm.
//!
//! Rather than require the browser host to supply a libc, these provide the
//! small subset that the run / type-check paths actually exercise, backed by
//! Rust's own global allocator and `core`. They are compiled **only** for wasm
//! (`#[cfg(target_arch = "wasm32")]`); the native build is completely
//! unaffected and continues to bind the platform libc.
//!
//! The allocator uses a size-prefixed block layout so that a bare `free(ptr)` /
//! `realloc(ptr, n)` (which carry no size) can recover the original allocation
//! size: each block reserves a `usize`-aligned header storing the user size,
//! and the returned pointer points just past it.

#![cfg(target_arch = "wasm32")]

use alloc::alloc::{alloc, dealloc, realloc as alloc_realloc};
use core::{
  alloc::Layout,
  ffi::{c_char, c_int, c_long, c_void},
  ptr::null_mut,
};

/// Bytes reserved before every user pointer to record the block's user size.
/// Sized/aligned to the maximum alignment we hand out (16 bytes) so the user
/// region is suitably aligned for any Luau value.
const HEADER: usize = 16;

/// wasm 页大小（`sysconf(_SC_PAGESIZE)` 的返回值）。
const WASM_PAGE_SIZE: c_long = 64 * 1024;

/// `malloc(size)` — allocate `size` bytes (size-prefixed).
///
/// # Safety
/// 仅 C ABI 边界；`size` 由调用方契约保证为请求字节数，本实现不读入参指针。
#[cfg(target_os = "unknown")] // wasi-libc provides malloc/free/realloc
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn malloc(size: usize) -> *mut c_void {
  unsafe {
    if size == 0 {
      // Return a unique, non-null, never-dereferenced pointer (C `malloc(0)`
      // is allowed to return either NULL or a freeable unique pointer; the
      // callers treat NULL as failure, so hand out a real 1-byte block).
      return malloc(1);
    }
    // 溢出回绕会产生过小的块（调用者按原 size 写入 → 堆溢出），按 C 约定
    // 返回 NULL。
    let total = match size.checked_add(HEADER) {
      Some(t) => t,
      None => return null_mut(),
    };
    let layout = match Layout::from_size_align(total, HEADER) {
      Ok(l) => l,
      Err(_) => return null_mut(),
    };
    let base = alloc(layout);
    if base.is_null() {
      return null_mut();
    }
    *(base as *mut usize) = size;
    base.add(HEADER) as *mut c_void
  }
}

/// `free(ptr)` — release a block previously returned by [`malloc`]/[`realloc`].
///
/// # Safety
/// `ptr` 必须是本模块分配且未释放的指针，或为 null。
#[cfg(target_os = "unknown")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn free(ptr: *mut c_void) {
  unsafe {
    if ptr.is_null() {
      return;
    }
    let base = (ptr as *mut u8).sub(HEADER);
    let size = *(base as *mut usize);
    let layout = Layout::from_size_align_unchecked(size + HEADER, HEADER);
    dealloc(base, layout);
  }
}

/// `realloc(ptr, size)` — resize a block, preserving its contents.
///
/// # Safety
/// `ptr` 必须是本模块分配且未释放的指针，或为 null。
#[cfg(target_os = "unknown")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
  unsafe {
    if ptr.is_null() {
      return malloc(size);
    }
    if size == 0 {
      free(ptr);
      return null_mut();
    }
    let base = (ptr as *mut u8).sub(HEADER);
    let old_size = *(base as *mut usize);
    let old_layout = Layout::from_size_align_unchecked(old_size + HEADER, HEADER);
    // 同 `malloc`：新大小溢出时返回 NULL 且保持原块不变。
    let new_total = match size.checked_add(HEADER) {
      Some(t) => t,
      None => return null_mut(),
    };
    let new_base = alloc_realloc(base, old_layout, new_total);
    if new_base.is_null() {
      return null_mut();
    }
    *(new_base as *mut usize) = size;
    new_base.add(HEADER) as *mut c_void
  }
}

/// `strchr(s, c)` — first occurrence of `c` in the NUL-terminated `s`, or NULL.
/// Matching the terminating NUL when `c == 0` is part of the C contract.
///
/// # Safety
/// `s` 必须指向以 NUL 结尾的缓冲区；`c` 按无符号字节解读。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn strchr(s: *const c_char, c: c_int) -> *mut c_char {
  unsafe {
    let target = c as u8 as c_char;
    let mut p = s;
    loop {
      let ch = *p;
      if ch == target {
        return p as *mut c_char;
      }
      if ch == 0 {
        return null_mut();
      }
      p = p.add(1);
    }
  }
}

/// `time(t)` — seconds since the Unix epoch. `wasm32-unknown-unknown` has no
/// clock syscall; rather than pull a JS host clock in as another import, this
/// reports a fixed reference instant. `os.time` / `os.date` therefore return a
/// stable value in the browser, which is fine for a playground (the scripts
/// shown do not depend on the wall clock). The result is also written through
/// `t` when non-null, matching the C signature.
///
/// # Safety
/// `t` 非空时必须可写一个 `i64`。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn time(t: *mut i64) -> i64 {
  // 2024-01-01T00:00:00Z — a fixed, deterministic reference instant.
  const REFERENCE_EPOCH: i64 = 1_704_067_200;
  if !t.is_null() {
    unsafe { *t = REFERENCE_EPOCH };
  }
  REFERENCE_EPOCH
}

/// `clock()` — processor time in `CLOCKS_PER_SEC` units. With no wasm clock
/// syscall available without an extra host import, this reports zero; `os.clock`
/// is only used for timing and is not load-bearing for the playground.
///
/// # Safety
/// 无参数；本实现不解引用任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn clock() -> c_long {
  0
}

/// `sysconf(name)` — only the page-size query is ever issued (by the JIT code
/// allocator, which is not executed in the interpreter path). Report the wasm
/// page size, see [`WASM_PAGE_SIZE`].
///
/// # Safety
/// 无参数；本实现不解引用任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sysconf(_name: c_int) -> c_long {
  WASM_PAGE_SIZE
}

// The page-mapping family is referenced only by the native-codegen page
// allocator, which is never reached on the interpreter-only wasm path. They are
// provided as faithful failures (mmap returns MAP_FAILED; the rest no-op) so
// the module links without a JS host having to supply them.

/// `mmap` — JIT page allocation; unreachable on the wasm interpreter path.
///
/// # Safety
/// 仅调用约定层面 unsafe；本实现不访问任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mmap(
  _addr: *mut c_void,
  _len: usize,
  _prot: c_int,
  _flags: c_int,
  _fd: c_int,
  _off: i64,
) -> *mut c_void {
  // MAP_FAILED == (void*)-1
  usize::MAX as *mut c_void
}

/// `munmap` — JIT page release; unreachable on the wasm interpreter path.
///
/// # Safety
/// 仅调用约定层面 unsafe；本实现不访问任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn munmap(_addr: *mut c_void, _len: usize) -> c_int {
  0
}

/// `mprotect` — JIT page permissions; unreachable on the wasm interpreter path.
///
/// # Safety
/// 仅调用约定层面 unsafe；本实现不访问任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mprotect(_addr: *mut c_void, _len: usize, _prot: c_int) -> c_int {
  0
}

/// Broken-down time, matching the C `struct tm` field layout/order the
/// translation's `os_date` uses on non-Windows targets (wasm included): the
/// nine C89 fields plus the BSD/glibc `tm_gmtoff`/`tm_zone` pair that the
/// pure-Rust `%z`/`%Z` rendering reads.
#[repr(C)]
pub struct Tm {
  pub tm_sec: c_int,
  pub tm_min: c_int,
  pub tm_hour: c_int,
  pub tm_mday: c_int,
  pub tm_mon: c_int,
  pub tm_year: c_int,
  pub tm_wday: c_int,
  pub tm_yday: c_int,
  pub tm_isdst: c_int,
  pub tm_gmtoff: c_long,
  pub tm_zone: *const c_char,
}

/// Convert a Unix timestamp to broken-down UTC time (civil calendar), shared by
/// [`gmtime_r`] and [`localtime_r`] (the wasm build has no timezone database,
/// so local time **is** UTC). Uses Howard Hinnant's well-known days-from-civil
/// inverse. The zone fields are pinned to match: `tm_gmtoff = 0` and
/// `tm_zone = "UTC"`, so `os.date("%z")` / `os.date("%Z")` render `+0000` /
/// `UTC` deterministically in the browser instead of lying about a local zone
/// that does not exist there.
///
/// # Safety
/// `result` 必须可写一个 `Tm`。
unsafe fn fill_tm(secs: i64, result: *mut Tm) {
  unsafe {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);

    (*result).tm_hour = (rem / 3600) as c_int;
    (*result).tm_min = ((rem % 3600) / 60) as c_int;
    (*result).tm_sec = (rem % 60) as c_int;

    // 1970-01-01 was a Thursday (wday 4).
    (*result).tm_wday = (((days % 7) + 4 + 7) % 7) as c_int;

    // days -> civil (y, m, d), m in [1,12], d in [1,31].
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let year = if m <= 2 { y + 1 } else { y };

    (*result).tm_year = (year - 1900) as c_int;
    (*result).tm_mon = (m - 1) as c_int;
    (*result).tm_mday = d as c_int;

    // tm_yday: days since Jan 1 of tm_year.
    let jan1 = days_from_civil(year, 1, 1);
    (*result).tm_yday = (days - jan1) as c_int;
    (*result).tm_isdst = 0;
    (*result).tm_gmtoff = 0;
    (*result).tm_zone = c"UTC".as_ptr();
  }
}

/// Days from 1970-01-01 to the given civil date (Hinnant's days_from_civil).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
  let y = if m <= 2 { y - 1 } else { y };
  let era = if y >= 0 { y } else { y - 399 } / 400;
  let yoe = y - era * 400;
  let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
  let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
  era * 146_097 + doe - 719_468
}

/// `gmtime_r(timep, result)` — UTC broken-down time.
///
/// # Safety
/// `timep`、`result` 非空时必须分别可读 / 可写一个 `i64` / `Tm`。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn gmtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm {
  unsafe {
    if timep.is_null() || result.is_null() {
      return null_mut();
    }
    fill_tm(*timep, result);
    result
  }
}

/// `localtime_r(timep, result)` — local broken-down time. The wasm build has no
/// timezone database, so local time is UTC.
///
/// # Safety
/// 同 [`gmtime_r`]。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm {
  unsafe { gmtime_r(timep, result) }
}
