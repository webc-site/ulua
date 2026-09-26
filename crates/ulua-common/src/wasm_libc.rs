//! `wasm32-unknown-unknown` 目标的最小 libc 面。
//!
//! `wasm32-unknown-unknown` 不携带 libc，忠实翻译经 `extern "C"` 声明的少数
//! C 函数（编译器的 C 风格分配器 `malloc`/`free`/`realloc`、类型检查器页
//! 分配器引用的 `sysconf`/`mmap` 族等）没有符号可绑定，否则会以未解析的
//! `env` import 出现在生成的 wasm 中。`string.format` 的 `snprintf` 与
//! `os.time`/`os.date` 的 `time`/`gmtime_r`/`localtime_r` 族都已不在 wasm
//! import 面上（格式化现为全目标纯 Rust 实现，见 `ulua-vm` 的
//! `format_directive`；时间分解同样全目标纯 Rust，见 `ulua-vm` 的
//! `localtime_r`/`os_date`——`jiff` 在无时区库目标回退 UTC，语义即旧
//! shim）。
//!
//! 与其要求浏览器宿主提供 libc，这里提供 run / type-check 路径实际用到的
//! 小子集，由 Rust 自己的全局分配器和 `core` 支撑。它们**只**为 wasm 编译
//! （`#[cfg(target_arch = "wasm32")]`）；native 构建完全不受影响，
//! 继续绑定平台 libc。
//!
//! 分配器用带大小前缀的块布局，使裸 `free(ptr)` / `realloc(ptr, n)`
//! （不带大小）能恢复原始分配大小：每块预留一个 `usize` 对齐的头部存用户
//! 大小，返回的指针指向其紧后方。
//!
//! rustify 边界评估：本模块内剩余的 `c_void` / `c_char` / `c_int` / `c_long`
//! 全部位于 C ABI 契约上——`#[unsafe(no_mangle)]` 签名（malloc/free/realloc/
//! mmap/strchr 等）。边界内部一律 Rust 化：`Layout` + `core::alloc` 做分配，
//! 字符串扫描收口到 `functions::c_str::cstr_bytes` 门面 + `memchr`（本模块
//! 不再直接 `CStr::from_ptr`）。
//! 本模块所有返回空指针处均属保留项：它们是各 C 接口契约规定的 NULL 回报
//! （malloc/realloc 分配失败、strchr 未命中、mmap 失败），
//! 返回类型本身即 `extern "C"` 裸指针签名，无 Option 可替换的空间。

// target_os = "unknown"（wasm32-unknown-unknown）整模块参与；wasi 系目标下
// malloc/free 等符号由 wasi-libc 提供，整模块门出以免重定义（cpp 无此面）
#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use alloc::alloc::{alloc, dealloc, realloc as alloc_realloc};
use core::{
  alloc::Layout,
  ffi::{c_char, c_int, c_long, c_void},
  ptr::null_mut,
};

use crate::functions::c_str::cstr_bytes;

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
  if size == 0 {
    // 返回唯一、非空、永不解引用的指针（C `malloc(0)` 允许返回 NULL 或
    // 可释放的唯一指针；调用方把 NULL 视为失败，故给一块真实的
    // 1 字节块）。
    return unsafe { malloc(1) };
  }
  // 溢出回绕会产生过小的块（调用者按原 size 写入 → 堆溢出），按 C 约定
  // 返回 NULL。
  let Some(total) = size.checked_add(HEADER) else {
    return null_mut();
  };
  // 对齐 HEADER 恒为 2 的幂，构造失败只可能是平台总量上限；同按 C 约定回 NULL。
  let Ok(layout) = Layout::from_size_align(total, HEADER) else {
    return null_mut();
  };
  // Safety: `layout` 由合法总大小与 2 的幂对齐构造，满足 alloc 的全部前提；
  // 返回空指针是 alloc 的失败形态，由下一分支处理。
  let base = unsafe { alloc(layout) };
  if base.is_null() {
    return null_mut();
  }
  // Safety: base 是刚由 `alloc(layout)` 取得、尚未交给调用方的块：
  // `layout.alignment() == HEADER ≥ size_of::<usize>()`，故块首写一个 `usize`
  // 落在头部界内；`layout.size() == size + HEADER`，故 `base.add(HEADER)` 不越界。
  unsafe {
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
  // Safety: `# Safety` 契约保证 `ptr` 是本模块 [`malloc`]/[`realloc`] 返回且未释放
  // 的指针（或 null，已提前返回）。这类指针恒为 `base.add(HEADER)`，故
  // `(ptr as *mut u8).sub(HEADER)` 回到分配起点，头里的 `size` 与 `alloc` 时一致，
  // `from_size_align_unchecked(size + HEADER, HEADER)` 复原的正是原 layout。
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
/// `ptr` 为 NULL 等价 `malloc(size)`；`size` 为 0 释放 `ptr` 并返回 NULL
/// （glibc 语义，native 构建同形）。VM 默认分配器 `l_alloc` 已改走
/// `std::alloc::System`，不再消费本垫片；本函数保留为 C ABI 分配器面
/// （`malloc`/`free` 配对契约的 resize 形态）与 wasm 分配器契约测试之用。
///
/// # Safety
/// `ptr` 必须是本模块分配且未释放的指针，或为 null。
#[cfg(target_os = "unknown")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
  if ptr.is_null() {
    return unsafe { malloc(size) };
  }
  if size == 0 {
    unsafe { free(ptr) };
    return null_mut();
  }
  // 同 `malloc`：新大小溢出时返回 NULL 且保持原块不变。
  let Some(new_total) = size.checked_add(HEADER) else {
    return null_mut();
  };
  // Safety: `# Safety` 契约保证 `ptr` 由本模块分配且未释放，故 `sub(HEADER)` 回到
  // 分配起点，头里的 `old_size` 是 alloc 时记录的用户大小，
  // `from_size_align_unchecked(old_size + HEADER, HEADER)` 复原的正是原 layout。
  let (base, old_layout) = unsafe {
    let base = (ptr as *mut u8).sub(HEADER);
    let old_size = *(base as *mut usize);
    (
      base,
      Layout::from_size_align_unchecked(old_size + HEADER, HEADER),
    )
  };
  // Safety: `base`/`old_layout` 即原分配的起点与 layout（上一块所证）；`new_total`
  // 经 checked_add 不回绕且对齐不变，满足 alloc_realloc 前提；失败返回空指针，
  // 由下一分支按 C 约定回 NULL 并保持原块不变。
  let new_base = unsafe { alloc_realloc(base, old_layout, new_total) };
  if new_base.is_null() {
    return null_mut();
  }
  // Safety: realloc 成功的新块与 [`malloc`] 块同形（HEADER 头 + 用户区），
  // 头部写一个 `usize` 与 `add(HEADER)` 均界内。
  unsafe {
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
  // Safety: 调用方契约保证 `s` 指向本调用期间存活的 NUL 结尾缓冲区，满足
  // `cstr_bytes` 前提；扫描交给 memchr 的 SIMD 实现，取代 C 风格的逐字节循环。
  let bytes = unsafe { cstr_bytes(s) };
  let needle = c as u8;
  let idx = if needle == 0 {
    // 字节切片天然不含结尾 NUL，其长度即结尾 NUL 的偏移（C 契约的命中点）。
    bytes.len()
  } else {
    match memchr::memchr(needle, bytes) {
      Some(i) => i,
      None => return null_mut(),
    }
  };
  // Safety: `idx ≤ strlen(s)`（memchr 命中点或字节切片长度），`add` 至多落在
  // 结尾 NUL 上，不越缓冲区界。
  unsafe { (s as *mut c_char).add(idx) }
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
/// `_name` 一律忽略（任何查询都回页大小，消费方 `page_size()` 对返回值
/// 二次校验「正且 2 的幂」）；本实现不解引用任何指针。
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
