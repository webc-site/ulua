//! `wasm32-unknown-unknown` 目标的最小 libc 面。
//!
//! `wasm32-unknown-unknown` 不携带 libc，忠实翻译经 `extern "C"` 声明的少数
//! C 函数（VM 的 C 风格分配器、`string.format` 的 `snprintf` 等）没有符号
//! 可绑定，否则会以未解析的 `env` import 出现在生成的 wasm 中。
//!
//! 与其要求浏览器宿主提供 libc，这里提供 run / type-check 路径实际用到的
//! 小子集，由 Rust 自己的全局分配器和 `core` 支撑。它们**只**为 wasm 编译
//! （`#[cfg(target_arch = "wasm32")]`）；native 构建完全不受影响，
//! 继续绑定平台 libc。
//!
//! 分配器用带大小前缀的块布局，使裸 `free(ptr)` / `realloc(ptr, n)`
//! （不带大小）能恢复原始分配大小：每块预留一个 `usize` 对齐的头部存用户
//! 大小，返回的指针指向其紧后方。

#![cfg(target_arch = "wasm32")]

use alloc::alloc::{alloc, dealloc, realloc as alloc_realloc};
use core::{
  alloc::Layout,
  ffi::{c_char, c_int, c_long, c_void},
  ptr::null_mut,
};

/// 每个用户指针前预留的字节数，记录块的用户大小。
/// 按我们给出的最大对齐（16 字节）取大小/对齐，使用户区域对任意
/// Luau 值都合适对齐。
const HEADER: usize = 16;

/// wasm 页大小（`sysconf(_SC_PAGESIZE)` 的返回值）。
const WASM_PAGE_SIZE: c_long = 64 * 1024;

/// `malloc(size)` — 分配 `size` 字节（带大小前缀）。
///
/// # Safety
/// 仅 C ABI 边界；`size` 由调用方契约保证为请求字节数，本实现不读入参指针。
#[cfg(target_os = "unknown")] // wasi-libc provides malloc/free/realloc
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn malloc(size: usize) -> *mut c_void {
  unsafe {
    if size == 0 {
      // 返回唯一、非空、永不解引用的指针（C `malloc(0)` 允许返回 NULL 或
      // 可释放的唯一指针；调用方把 NULL 视为失败，故给一块真实的
      // 1 字节块）。
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

/// `free(ptr)` — 释放先前由 [`malloc`]/[`realloc`] 返回的块。
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

/// `realloc(ptr, size)` — 调整块大小，保留内容。
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

/// `strchr(s, c)` — `c` 在 NUL 结尾 `s` 中的首次出现，无则 NULL。
/// `c == 0` 时匹配结尾 NUL 是 C 契约的一部分。
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

/// `time(t)` — 自 Unix 纪元起的秒数。`wasm32-unknown-unknown` 没有时钟
/// syscall；与其再引入一个 JS 宿主时钟 import，这里报告固定的参考时刻。
/// `os.time` / `os.date` 因此在浏览器里返回稳定值，对 playground 足够
/// （展示的脚本不依赖挂钟）。`t` 非空时结果也经 `t` 写出，匹配 C 签名。
///
/// # Safety
/// `t` 非空时必须可写一个 `i64`。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn time(t: *mut i64) -> i64 {
  // 2024-01-01T00:00:00Z — 固定的确定性参考时刻。
  const REFERENCE_EPOCH: i64 = 1_704_067_200;
  if !t.is_null() {
    unsafe { *t = REFERENCE_EPOCH };
  }
  REFERENCE_EPOCH
}

/// `clock()` — 处理器时间（`CLOCKS_PER_SEC` 单位）。没有额外宿主 import
/// 就没有可用的 wasm 时钟 syscall，这里报告零；`os.clock` 只用于计时，
/// 对 playground 不承重。
///
/// # Safety
/// 无参数；本实现不解引用任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn clock() -> c_long {
  0
}

/// `sysconf(name)` — 只有页大小查询会被发起（由 JIT 代码分配器发起，
/// 解释器路径不执行）。报告 wasm 页大小，见 [`WASM_PAGE_SIZE`]。
///
/// # Safety
/// 无参数；本实现不解引用任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sysconf(_name: c_int) -> c_long {
  WASM_PAGE_SIZE
}

// 页映射族只被 native-codegen 页分配器引用，解释器-only 的 wasm 路径
// 永远到达不了。以"忠实失败"提供（mmap 返回 MAP_FAILED；其余 no-op），
// 使模块无需 JS 宿主提供即可链接。

/// `mmap` — JIT 页分配；wasm 解释器路径不可达。
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

/// `munmap` — JIT 页释放；wasm 解释器路径不可达。
///
/// # Safety
/// 仅调用约定层面 unsafe；本实现不访问任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn munmap(_addr: *mut c_void, _len: usize) -> c_int {
  0
}

/// `mprotect` — JIT 页权限；wasm 解释器路径不可达。
///
/// # Safety
/// 仅调用约定层面 unsafe；本实现不访问任何指针。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mprotect(_addr: *mut c_void, _len: usize, _prot: c_int) -> c_int {
  0
}

/// 分解时间，匹配翻译的 `os_date` 在非 Windows 目标（含 wasm）使用的
/// C `struct tm` 字段布局/顺序：C89 的九个字段加上 BSD/glibc 的
/// `tm_gmtoff`/`tm_zone` 对，纯 Rust 的 `%z`/`%Z` 渲染会读它们。
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

/// 把 Unix 时间戳转为分解 UTC 时间（civil 历法），[`gmtime_r`] 与
/// [`localtime_r`] 共用（wasm 构建没有时区数据库，本地时间**就是** UTC）。
/// 用 Howard Hinnant 的著名 days-from-civil 逆变换。zone 字段固定匹配：
/// `tm_gmtoff = 0` 且 `tm_zone = "UTC"`，故 `os.date("%z")` /
/// `os.date("%Z")` 在浏览器里确定性地渲染 `+0000` / `UTC`，而不是谎报
/// 一个那里不存在的本地时区。
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

    // 1970-01-01 是星期四（wday 4）。
    (*result).tm_wday = (((days % 7) + 4 + 7) % 7) as c_int;

    // days -> civil (y, m, d)，m ∈ [1,12]，d ∈ [1,31]。
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

    // tm_yday：自 tm_year 的 1 月 1 日起的天数。
    let jan1 = days_from_civil(year, 1, 1);
    (*result).tm_yday = (days - jan1) as c_int;
    (*result).tm_isdst = 0;
    (*result).tm_gmtoff = 0;
    (*result).tm_zone = c"UTC".as_ptr();
  }
}

/// 自 1970-01-01 到给定 civil 日期的天数（Hinnant 的 days_from_civil）。
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
  let y = if m <= 2 { y - 1 } else { y };
  let era = if y >= 0 { y } else { y - 399 } / 400;
  let yoe = y - era * 400;
  let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
  let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
  era * 146_097 + doe - 719_468
}

/// `gmtime_r(timep, result)` — UTC 分解时间。
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

/// `localtime_r(timep, result)` — 本地分解时间。wasm 构建没有时区数据库，
/// 本地时间就是 UTC。
///
/// # Safety
/// 同 [`gmtime_r`]。
///
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm {
  unsafe { gmtime_r(timep, result) }
}
