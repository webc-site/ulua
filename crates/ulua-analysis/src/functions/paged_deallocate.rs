use alloc::alloc::dealloc;
use core::alloc::Layout;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::functions::paged_allocate::page_size;

#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
unsafe extern "C" {
  fn munmap(addr: *mut u8, len: usize) -> i32;
}

/// Port of `Luau::pagedDeallocate`.
///
/// Frees a block previously returned by `paged_allocate`. `size` is always the
/// block size the matching allocation was made with (`K_BLOCK_SIZE_BYTES`), which
/// lets the default heap path reconstruct the same `Layout`.
pub(crate) fn paged_deallocate(ptr: *mut u8, size: usize, freeze: bool) {
  // `freeze` is the strategy the matching `paged_allocate` used (captured once by
  // the owning TypedAllocator). It must match the allocation — re-reading the
  // global DebugLuauFreezeArena flag here was the bug: it is a toggleable
  // ScopedFastFlag, so a block allocated under one value could be freed under the
  // other, mismatching VirtualFree/operator-delete and corrupting the heap (the
  // Windows 0xC0000005 / VirtualFree==0 failures).
  if !freeze {
    // `::operator delete(ptr)`. Reconstruct the exact `Layout` used by
    // `paged_allocate`'s default branch.
    if ptr.is_null() || size == 0 {
      return;
    }
    if let Ok(layout) = Layout::from_size_align(size, page_size()) {
      // Safety: 与 `paged_allocate` 默认分支 `alloc(layout)` 严格配对——`ptr` 非空
      // （上方 is_null/size==0 已早退，故为成功分配返回的堆块），`layout` 由相同的
      // 「非零 size + 2 的幂页对齐」重建，与被释放块当初的分配 Layout 完全一致。
      unsafe { dealloc(ptr, layout) };
    }
    return;
  }

  #[cfg(target_os = "windows")]
  {
    // The matching `paged_allocate` freeze-path used `VirtualAlloc`, so this
    // block is OS virtual memory and MUST be released with `VirtualFree(...,
    // MEM_RELEASE)`. The previous `_aligned_free` (a CRT-heap function) was
    // called on `VirtualAlloc`'d memory — a catastrophic allocator mismatch
    // that corrupted the heap and crashed ~every type-checking test on Windows
    // with 0xC0000005 (the test fixtures set DebugLuauFreezeArena=true, so all
    // their type arenas take this path). Linux/macOS were correct (mmap/munmap),
    // which is why the bug was Windows-only and invisible to valgrind on Linux.
    // MEM_RELEASE requires the size argument to be 0.
    use windows_sys::Win32::System::Memory::{MEM_RELEASE, VirtualFree};
    let _ = size;
    // Safety: 冻结路径 `freeze==true` 时，本块由 `paged_allocate` 的 Windows 分支用
    // `VirtualAlloc` 分配并返回页对齐基址；`ptr` 即该存活映射基址，`VirtualFree(p,0,
    // MEM_RELEASE)` 是 C++ `pagedDeallocate` 的同款释放（MEM_RELEASE 约定 dwSize 必为 0）。
    let rc = unsafe { VirtualFree(ptr, 0, MEM_RELEASE) };
    LUAU_ASSERT!(rc != 0);
  }

  #[cfg(target_os = "freebsd")]
  {
    unsafe extern "C" {
      fn free(ptr: *mut u8);
    }
    // Safety: 冻结路径在 FreeBSD 上由 `paged_allocate` 以 `aligned_alloc` 分配，`ptr`
    // 为其返回的存活堆块（非空）；libc `free` 接受该指针，与 C++ `pagedDeallocate` 同形。
    unsafe { free(ptr) };
  }

  #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
  {
    // Safety: 对 libc `munmap` 的裸 FFI：`freeze==true` 时本块由 `paged_allocate` 的
    // POSIX 分支用 `mmap` 返回，`ptr` 为该存活映射基址（非空、页对齐），`size` 是拥有者
    // 记录的同一块大小（`K_BLOCK_SIZE_BYTES`），与 C++ `pagedDeallocate` 的 `munmap(ptr,size)`
    // 完全一致；二者均按页粒度处理，返回码在下方断言为 0。
    let rc = unsafe { munmap(ptr, size) };
    LUAU_ASSERT!(rc == 0);
  }
}
