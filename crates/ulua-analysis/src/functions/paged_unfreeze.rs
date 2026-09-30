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
    let rc = unsafe { mprotect(ptr, page_align(size), PROT_READ | PROT_WRITE) };
    LUAU_ASSERT!(rc == 0);
  }
}
