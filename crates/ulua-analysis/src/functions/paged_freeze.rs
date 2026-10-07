use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::functions::paged_allocate::{page_align, page_size};

#[cfg(not(target_os = "windows"))]
unsafe extern "C" {
  fn mprotect(addr: *mut u8, len: usize, prot: i32) -> i32;
}

/// Port of `Luau::pagedFreeze`.
///
/// Marks `[ptr, ptr + pageAlign(size))` read-only so that any mutation of a
/// frozen arena traps. Only valid when `DebugLuauFreezeArena` is set and `ptr`
/// is page aligned.
pub(crate) fn paged_freeze(ptr: *mut u8, size: usize) {
  LUAU_ASSERT!(fflag::DebugLuauFreezeArena.get());
  LUAU_ASSERT!((ptr as usize).is_multiple_of(page_size()));

  #[cfg(target_os = "windows")]
  {
    use windows_sys::Win32::System::Memory::{PAGE_READONLY, VirtualProtect};
    let mut old_protect: u32 = 0;
    // SAFETY: 调用点保证 `ptr` 页对齐、`page_align(size)` 为本 arena 独占的
    // 已映射可读写区域；单线程驱动且改读写保护不产生 Rust 引用别名冲突。
    let rc = unsafe {
      VirtualProtect(
        ptr.cast(),
        page_align(size),
        PAGE_READONLY,
        &mut old_protect,
      )
    };
    LUAU_ASSERT!(rc != 0);
  }

  #[cfg(not(target_os = "windows"))]
  {
    const PROT_READ: i32 = 0x1;
    // SAFETY: 同 windows 分支——`ptr` 页对齐、区域映射有效且本 arena 独占，
    // `mprotect` 仅改页保护位，单线程串行、无并存 Rust 可变别名。
    let rc = unsafe { mprotect(ptr, page_align(size), PROT_READ) };
    LUAU_ASSERT!(rc == 0);
  }
}
