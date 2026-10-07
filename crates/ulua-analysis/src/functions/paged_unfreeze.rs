use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::functions::paged_allocate::{page_align, page_size};

#[cfg(not(target_os = "windows"))]
unsafe extern "C" {
  fn mprotect(addr: *mut u8, len: usize, prot: i32) -> i32;
}

/// Port of `Luau::pagedUnfreeze`.
///
/// Restores read-write access to `[ptr, ptr + pageAlign(size))` previously
/// frozen by `paged_freeze`. Only valid when `DebugLuauFreezeArena` is set and
/// `ptr` is page aligned.
pub(crate) fn paged_unfreeze(ptr: *mut u8, size: usize) {
  LUAU_ASSERT!(fflag::DebugLuauFreezeArena.get());
  LUAU_ASSERT!((ptr as usize).is_multiple_of(page_size()));

  #[cfg(target_os = "windows")]
  {
    use windows_sys::Win32::System::Memory::{PAGE_READWRITE, VirtualProtect};
    let mut old_protect: u32 = 0;
    // SAFETY: 调用点保证 `ptr` 页对齐、`page_align(size)` 为先前 paged_freeze
    // 冻结的同一映射区域；单线程驱动且改读写保护不产生 Rust 引用别名冲突。
    let rc = unsafe {
      VirtualProtect(
        ptr.cast(),
        page_align(size),
        PAGE_READWRITE,
        &mut old_protect,
      )
    };
    LUAU_ASSERT!(rc != 0);
  }

  #[cfg(not(target_os = "windows"))]
  {
    const PROT_READ: i32 = 0x1;
    const PROT_WRITE: i32 = 0x2;
    // SAFETY: 同 windows 分支——`ptr` 页对齐、区域为此前 paged_freeze 冻结的
    // 同一映射，`mprotect` 仅恢复读写页保护位，单线程串行、无并存可变别名。
    let rc = unsafe { mprotect(ptr, page_align(size), PROT_READ | PROT_WRITE) };
    LUAU_ASSERT!(rc == 0);
  }
}
