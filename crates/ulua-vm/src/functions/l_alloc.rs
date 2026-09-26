use core::{ffi::c_void, ptr::null_mut};
use std::alloc::{GlobalAlloc, Layout, System};

/// 默认分配块的声明对齐：`System` 在 align ≤ 16 时直通平台分配器（malloc 系），
/// 实际返回地址天然满对齐，Layout 只须在本函数 alloc/dealloc/realloc 三条路径间自洽。
const DEFAULT_ALIGN: usize = 1;

/// # Safety
/// `ud` 为 `lua_newstate` 传入的分配器用户指针（可空）；`ptr` 为 null 或本分配器此前返回的块，
/// `osize` 为 0 表示纯分配、否则旧块可读。
///
/// lua.h 契约分支：
/// - `ptr == null`（此时契约保证 `osize == 0`）：`nsize == 0` 无事可做返回 null，否则纯分配；
/// - `ptr != null`、`nsize == 0`：释放并返回 null（`osize` 为原块大小，用于复原 Layout）；
/// - `ptr != null`、`nsize != 0`：缩/扩块，C realloc 语义下原块大小即 `osize`，
///   故 `Layout::from_size_align(osize, DEFAULT_ALIGN)` 正是分配时的布局。
///
/// 内部实现走 `std::alloc::System`（平台分配器的稳定封装），不再声明 extern libc
/// `free`/`realloc`：native 路径与之等价（`System` 在 `DEFAULT_ALIGN` 下即 malloc/free/realloc），
/// `wasm32-unknown-unknown` 路径由 std 运行时直接落到 Rust 分配器，无需 `wasm_libc` 垫片参与。
pub unsafe extern "C-unwind" fn l_alloc(
  ud: *mut c_void,
  ptr: *mut u8,
  osize: usize,
  nsize: usize,
) -> *mut u8 {
  let _ = ud;

  if ptr.is_null() {
    // 纯分配路径；size==0 的 Layout 非法（且契约上无事可做），直接回报 null
    let Some(layout) = block_layout(nsize) else {
      return null_mut();
    };
    // Safety: `layout` 大小非零、对齐为 1（2 的幂），满足 GlobalAlloc::alloc 前提；
    // 失败时 malloc 回 null，由本函数原样上交，调用方按 lua.h 契约视为分配失败。
    unsafe { System.alloc(layout) }
  } else if nsize == 0 {
    // 释放路径：`osize` 即本块分配时的大小（lua.h 契约）；契约违例（osize==0）
    // 时无从复原 Layout，只回 null 不调用 dealloc，避免未定义行为。
    if let Some(layout) = block_layout(osize) {
      // Safety: `ptr` 由契约保证是本分配器以同参数 `block_layout` 交回且未释放的块，
      // `layout` 与分配时逐项一致，满足 GlobalAlloc::dealloc 前提。
      unsafe { System.dealloc(ptr, layout) };
    }
    null_mut()
  } else {
    // 缩/扩路径：原块 Layout 由 `osize` 复原（见函数 `# Safety` 说明）。
    let Some(old) = block_layout(osize) else {
      return null_mut();
    };
    // Safety: `ptr`/`old` 同上契约；`nsize != 0` 已在本分支入口保证（为 0 走上一个
    // 释放分支），满足 GlobalAlloc::realloc 的全部前提。失败返回 null 表示未移动，
    // 原块仍存活，由调用方按分配失败处理。
    unsafe { System.realloc(ptr, old, nsize) }
  }
}

/// 由大小构造默认 Layout；size==0（Layout 非法）或参数溢出时回 None。
#[inline]
fn block_layout(size: usize) -> Option<Layout> {
  if size == 0 {
    return None;
  }
  Layout::from_size_align(size, DEFAULT_ALIGN).ok()
}
