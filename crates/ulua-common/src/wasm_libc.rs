//! `wasm32-unknown-unknown` 目标的最小 libc 面。
//!
//! `wasm32-unknown-unknown` 不携带 libc，忠实翻译经 `extern "C"` 声明的少数
//! C 函数（类型检查器页 arena 与 JIT 页分配器引用的页大小/页映射族
//! `sysconf`/`mmap`/`munmap`/`mprotect`）没有符号可绑定，否则会以未解析的
//! `env` import 出现在生成的 wasm 中。`string.format` 的
//! `snprintf`、`os.time`/`os.date` 的 `time`/`gmtime_r`/`localtime_r` 族、
//! `os.clock` 的 `clock` 与字符串扫描的 `strchr` 都已不在 wasm import 面上
//! （格式化现为全目标纯 Rust 实现，见 `ulua-vm` 的 `format_directive`；时间
//! 分解同样全目标纯 Rust，见 `ulua-vm` 的 `localtime_r`/`os_date`——`jiff` 在
//! 无时区库目标回退 UTC，语义即旧 shim；单调计时全目标统一走
//! `clock_shim::monotonic_seconds`，`os_clock` 不经 C `clock`；`strchr` 的
//! 扫描面已换 `cstr_bytes`+`memchr`）。`clock`/`strchr` 两枚因全仓无任何
//! `extern "C"` 声明导入（strtod 垫片于 b22 退役的同一判据，先例见
//! `strtod_shim.rs` 模块头注记）于本轮（r10）退役。
//!
//! 与其要求浏览器宿主提供 libc，这里提供 run / type-check 实际消费的
//! 小子集，由 Rust 自己的全局分配器和 `core` 支撑。现存七枚的 extern
//! 消费面（按实态逐一核对）：`sysconf` ← `ulua-analysis`
//! `paged_allocate.rs:45` `page_size()`；`mmap` ← `paged_allocate.rs:8` 与
//! `ulua-code-gen` `allocate_pages_impl_code_allocator.rs:8`；`munmap` ←
//! `paged_deallocate.rs:10` 与 `free_pages_impl_code_allocator.rs:41`；
//! `mprotect` ← `paged_freeze.rs:7`/`paged_unfreeze.rs:7` 与 code-gen
//! `make_pages_*` 族。分配器对 `malloc`/`free` 与 resize 形态 `realloc` 系
//! C 分配器配对契约面：native 侧 extern 消费方为 `paged_deallocate.rs:63`
//! （freebsd 堆释放）与 conformance `c_alloc.rs:7-8`（均绑定平台 libc，
//! 不到达本垫片）；wasm 链接面上无 importer，由本 crate
//! `tests/wasm_libc.rs` 的 `allocator_roundtrip` 契约用例钉住保留。
//! 它们**只**为 wasm 编译（`#[cfg(target_arch = "wasm32")]`）；native 构建
//! 完全不受影响，继续绑定平台 libc。
//!
//! 分配器用带大小前缀的块布局，使裸 `free(ptr)` / `realloc(ptr, n)`
//! （不带大小）能恢复原始分配大小：每块预留一个 `usize` 对齐的头部存用户
//! 大小，返回的指针指向其紧后方。
//!
//! rustify 边界评估：本模块内剩余的 `c_void` / `c_int` / `c_long`
//! 全部位于 C ABI 契约上——`#[unsafe(no_mangle)]` 签名（malloc/free/realloc/
//! mmap/mprotect 等）。边界内部一律 Rust 化：`Layout` + `core::alloc` 做分配；
//! 字符串扫描面（原 `strchr`）已收口到 `functions::c_str::cstr_bytes`
//! 门面 + `memchr`（本模块不再直接解引用宿主 C 串指针，该消费者退役后
//! 本模块亦不再引用 `cstr_bytes`）。
//! 本模块所有返回空指针处均属保留项：它们是各 C 接口契约规定的 NULL 回报
//! （malloc/realloc 分配失败、mmap 失败），
//! 返回类型本身即 `extern "C"` 裸指针签名，无 Option 可替换的空间。
//! r2 收口形态：块布局（头部读写、base↔user 换算、layout 复原）的**全部**
//! `unsafe` 挤进 [`alloc_block`]/[`user_of`]/[`block_of`] 三枚最小 `# Safety`
//! 封装，`extern "C"` 面只剩契约适配（`Option`/`Layout` 编排 + 单点转调），
//! 不再有逐函数的指针算术。

// target_os = "unknown"（wasm32-unknown-unknown）整模块参与；wasi 系目标下
// malloc/free 等符号由 wasi-libc 提供，整模块门出以免重定义（cpp 无此面）
#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use alloc::alloc::{alloc, dealloc, realloc as alloc_realloc};
use core::{
  alloc::Layout,
  ffi::{c_int, c_long, c_void},
  ptr::{NonNull, null_mut},
};

/// 每个用户指针前预留的字节数，记录块的用户大小。
/// 按我们给出的最大对齐（16 字节）取大小/对齐，使用户区域对任意
/// Luau 值都合适对齐。
const HEADER: usize = 16;

/// wasm 页大小（`sysconf(_SC_PAGESIZE)` 的返回值）。
const WASM_PAGE_SIZE: c_long = 64 * 1024;

/// 「用户大小 + 头部」的总布局唯一定义面：溢出回绕与平台上限都译成 `None`
/// （对应 C 约定的 NULL 回报）。分配与释放两侧同经此形，钉死 layout 一致性。
fn layout_of(size: usize) -> Option<Layout> {
  Layout::from_size_align(size.checked_add(HEADER)?, HEADER).ok()
}

/// 分配 + 写头 + 交用户指针的构造面（[`malloc`] 与 [`realloc`] 成功路径共用）：
/// 溢出/上限/分配失败统一回 `None`（C 约定 NULL）。除本面与 [`user_of`]/[`block_of`]
/// 外，模块内不再触碰分配器与块布局。
fn alloc_block(size: usize) -> Option<NonNull<c_void>> {
  let layout = layout_of(size)?;
  // Safety: `layout` 非零大小、对齐为 2 的幂，满足 alloc 全部前提；
  // 空指针是分配失败形态，经 NonNull::new 归一为 None。
  let base = NonNull::new(unsafe { alloc(layout) })?;
  // Safety: `base` 是刚由 alloc(layout) 独占取得、尚未交付的块：总大小
  // `size + HEADER`、对齐 HEADER，[`user_of`] 的两条界内前提均成立。
  Some(unsafe { user_of(base, size) })
}

/// 块起点 → 用户指针的收尾形态：写入大小头部并返回紧后方的用户区起点。
///
/// # Safety
/// `base` 必须是 `HEADER` 对齐、共 `size + HEADER` 字节的合法存活块起点
/// （此时头部写一个 `usize` 与 `HEADER` 偏移均落在块界内）。
unsafe fn user_of(base: NonNull<u8>, size: usize) -> NonNull<c_void> {
  // Safety: 前置条件即上述契约——对齐 HEADER ≥ size_of::<usize>()，
  // 总大小含头部，写头与偏移均界内。
  unsafe {
    base.cast::<usize>().write(size);
    base.byte_add(HEADER).cast()
  }
}

/// 用户指针 → (分配起点, 原 layout) 的唯一复原面：读头部取用户大小。
///
/// # Safety
/// `user` 必须是本模块 [`alloc_block`]（即 `malloc`/`realloc`）返回且未释放的
/// 非空用户指针（头部完好）。
unsafe fn block_of(user: NonNull<c_void>) -> (NonNull<u8>, Layout) {
  // Safety: 契约保证 `user == base + HEADER`，`byte_sub` 回到分配起点；头部恒由
  // [`user_of`] 以当时的用户大小写入，`size + HEADER` 即 alloc/realloc 时的总大小、
  // 对齐恒 HEADER，故复原出的正是原 layout（`from_size_align_unchecked` 界内成立）。
  unsafe {
    let base = user.cast::<u8>().byte_sub(HEADER);
    let size = base.cast::<usize>().read();
    (
      base,
      Layout::from_size_align_unchecked(size + HEADER, HEADER),
    )
  }
}

/// `malloc(size)` — 分配 `size` 字节（带大小前缀）。
///
/// # Safety
/// 仅 C ABI 边界；`size` 由调用方契约保证为请求字节数，本实现不读入参指针。
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn malloc(size: usize) -> *mut c_void {
  // C `malloc(0)` 允许返回 NULL 或可释放的唯一指针，而调用方把 NULL 视为失败：
  // 统一以 1 字节真实块回应（免原实现的自递归重入形态）。
  // FFI: malloc 契约要求分配失败回报 NULL（返回类型即 `extern "C"` 裸指针，无 Option 空间）。
  alloc_block(size.max(1)).map_or(null_mut(), NonNull::as_ptr)
}

/// `free(ptr)` — 释放先前由 [`malloc`]/[`realloc`] 返回的块。
///
/// # Safety
/// `ptr` 必须是本模块分配且未释放的指针，或为 null。
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn free(ptr: *mut c_void) {
  // free(NULL) 按 C 约定为 no-op，空指针不进入释放面
  let Some(user) = NonNull::new(ptr) else {
    return;
  };
  // Safety: `# Safety` 契约保证非空 `ptr` 是本模块分配且未释放的块，
  // 即 [`block_of`] 的前置条件。
  let (base, layout) = unsafe { block_of(user) };
  // Safety: `base`/`layout` 即原分配的起点与 layout（上一行契约所证）。
  unsafe { dealloc(base.as_ptr(), layout) };
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
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
  let Some(user) = NonNull::new(ptr) else {
    // C 约定：realloc(NULL, n) 与 malloc(n) 等价（含 n==0 的唯一指针形态）
    return unsafe { malloc(size) };
  };
  if size == 0 {
    // FFI: glibc 约定释放该块并回报 NULL（free 的 null 分支此处不可达）
    unsafe { free(ptr) };
    return null_mut();
  }
  // FFI: 新总大小溢出/达上限，按 C 契约回 NULL 且保持原块不变。
  let Some(new_layout) = layout_of(size) else {
    return null_mut();
  };
  // Safety: `# Safety` 契约保证 `ptr` 由本模块分配且未释放，即 [`block_of`] 前置。
  let (base, old_layout) = unsafe { block_of(user) };
  // Safety: `base`/`old_layout` 即原分配的起点与 layout（block_of 所证）；
  // `new_layout` 与 alloc_block 同形（对齐 HEADER、总大小经 checked_add 不回绕），
  // 满足 alloc_realloc 前提；FFI: C 契约要求分配失败回 NULL 且保留原块。
  let Some(new_base) =
    NonNull::new(unsafe { alloc_realloc(base.as_ptr(), old_layout, new_layout.size()) })
  else {
    return null_mut();
  };
  // Safety: 成功的新块与 alloc_block 块同形（HEADER 对齐、总大小含头部），
  // [`user_of`] 的界内前提成立。
  unsafe { user_of(new_base.cast(), size) }.as_ptr()
}

/// `sysconf(name)` — 页大小查询。全仓唯一 extern 消费方为 `ulua-analysis`
/// `paged_allocate.rs:45` 的 `page_size()`（类型检查 arena 页对齐，wasm 链接
/// 面上同样参与）；`ulua-code-gen` 侧页大小恒取 4096、不经此。报告 wasm 页
/// 大小，见 [`WASM_PAGE_SIZE`]。`_name`
/// 一律忽略（任何查询都回页大小，消费方 `page_size()` 对返回值二次校验
/// 「正且 2 的幂」）；全函数不解引用任何指针，无前置条件。
#[unsafe(no_mangle)]
pub extern "C-unwind" fn sysconf(_name: c_int) -> c_long {
  WASM_PAGE_SIZE
}

// 页映射族被两族 extern 面引用：native-codegen 页分配器，以及 ulua-analysis
// paged_* 族（`mmap` 实据 `paged_allocate.rs:8`；`munmap` ← `paged_deallocate.rs:10`；
// `mprotect` ← `paged_freeze.rs:7`/`paged_unfreeze.rs:7`）。analysis 侧 extern 块
// 的门为「非 windows/非 freebsd」，wasm 链接面上同样参与；但其默认路径
// （freeze=false）走 Rust 堆分配器，仅 `DebugLuauFreezeArena` 调试开关下到达
// mmap。以"忠实失败"提供（mmap 返回 MAP_FAILED，消费方归一为分配失败；
// 其余 no-op），使模块无需 JS 宿主提供即可链接。

/// `mmap` — JIT 页分配与 analysis paged_* 冻结路径共用同一 extern 面；
/// wasm 默认路径不可达。忠实失败形态：恒返回
/// MAP_FAILED，不访问任何指针，全函数无前置条件。
#[unsafe(no_mangle)]
pub extern "C-unwind" fn mmap(
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

/// `munmap` — JIT 页释放；wasm 解释器路径不可达。no-op：恒返回 0，
/// 不访问任何指针，全函数无前置条件。
#[unsafe(no_mangle)]
pub extern "C-unwind" fn munmap(_addr: *mut c_void, _len: usize) -> c_int {
  0
}

/// `mprotect` — JIT 页权限；wasm 解释器路径不可达。no-op：恒返回 0，
/// 不访问任何指针，全函数无前置条件。
#[unsafe(no_mangle)]
pub extern "C-unwind" fn mprotect(_addr: *mut c_void, _len: usize, _prot: c_int) -> c_int {
  0
}
