use alloc::alloc::alloc;
#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
use core::ptr::null_mut;
use core::{alloc::Layout, ptr::NonNull};

#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
unsafe extern "C" {
  fn mmap(
    addr: *mut u8,
    len: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    // `off_t` is 64-bit on the LP64 native targets we support, and the wasm
    // `wasm_libc` shim declares it `i64` too — `isize` would be `i32` on
    // wasm32, a signature mismatch that breaks the wasm link. Use `i64`.
    offset: i64,
  ) -> *mut u8;
}

/// The OS page size.
///
/// Mirrors C++ `kPageSize`: `sysconf(_SC_PAGESIZE)` on POSIX, `getpagesize()`
/// on FreeBSD, 4096 on Win32.
#[cfg(not(target_os = "windows"))]
pub(crate) fn page_size() -> usize {
  // Under Miri there is no real OS page table and `sysconf` is an unsupported
  // foreign call; a fixed 4 KiB page (a valid power-of-two alignment) lets the
  // UB checker exercise the arena via the std allocator path below.
  #[cfg(miri)]
  {
    return 4096;
  }

  #[cfg(target_os = "freebsd")]
  {
    unsafe extern "C" {
      fn getpagesize() -> i32;
    }
    unsafe { getpagesize() as usize }
  }

  #[cfg(not(target_os = "freebsd"))]
  {
    unsafe extern "C" {
      fn sysconf(name: i32) -> isize;
    }
    // `_SC_PAGESIZE` is NOT the same number on every platform: it is 30 on
    // Linux/Android but 29 on macOS/Darwin. The constant `29` here was the
    // Darwin value — on Linux 29 is `_SC_VERSION`, so `sysconf(29)` returned
    // the POSIX version (~200809), which is not a power of two and made every
    // arena `Layout::from_size_align` fail (→ paged_allocate returns null →
    // the allocator panics → the whole type checker falls over on Linux).
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const _SC_PAGESIZE: i32 = 30;
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    const _SC_PAGESIZE: i32 = 29;
    // Safety: libc `sysconf` 为纯查询 C ABI，标量入参、不解引用指针；`_SC_PAGESIZE`
    // 已按目标平台取值（见上方注释），返回值为 POSIX 契约内的 isize；即便返回异常值，
    // 下方仍二次校验「正且 2 的幂」，否则回落 4096。
    let v = unsafe { sysconf(_SC_PAGESIZE) };
    // Defensive: a page size must be a positive power of two (it is the
    // alignment we hand to `Layout`). If sysconf ever returns something
    // unexpected, fall back to 4 KiB rather than abort every allocation.
    if v >= 1 && (v as usize).is_power_of_two() {
      v as usize
    } else {
      4096
    }
  }
}

#[cfg(target_os = "windows")]
pub(crate) fn page_size() -> usize {
  4096
}

/// Round `size` up to a multiple of the OS page size.
///
/// Mirrors C++ `pageAlign`: `(size + kPageSize - 1) & ~(kPageSize - 1)`.
pub(crate) fn page_align(size: usize) -> usize {
  let page = page_size();
  (size + page - 1) & !(page - 1)
}

/// Port of `Luau::pagedAllocate`.
///
/// By default we use operator new/delete instead of malloc/free so that they
/// can be overridden externally. When `DebugLuauFreezeArena` is set, we use
/// page-granular OS allocation so the blocks can later be frozen with
/// `mprotect`/`VirtualProtect`.
///
/// 返回 `Option<NonNull<u8>>`（review.md §2 (a)）：`None` 即原 cpp
/// `operator new(nothrow)`/`mmap` 失败返回 `nullptr` 的「分配失败」哨兵，唯一消费方
/// `TypedAllocator::append_block` 将其折回 `bad_alloc` panic；成功块的释放仍走
/// `paged_deallocate` 的裸指针契约（块地址由 arena 独占持有）。
pub fn paged_allocate(size: usize, freeze: bool) -> Option<NonNull<u8>> {
  // `freeze` is the allocation strategy chosen by the *caller* (the owning
  // TypedAllocator captures `DebugLuauFreezeArena` once, at its first
  // allocation, and threads the same value into both allocate and deallocate).
  // It must NOT be re-read from the global flag here: the flag is a toggleable
  // ScopedFastFlag in tests, so reading it at free time could pick a different
  // strategy than was used to allocate — e.g. VirtualFree on heap memory or
  // operator delete on VirtualAlloc memory — corrupting the heap. That mismatch
  // is what crashed ~all type-checking tests on Windows (0xC0000005 / VirtualFree
  // returning 0 at paged_deallocate). See `paged_deallocate`.
  if !freeze {
    // `::operator new(size, std::nothrow)` — a heap allocation that returns
    // null on failure. The matching `::operator delete` lives in
    // `paged_deallocate`; both reconstruct the identical `Layout` from the
    // size the caller passes (always `K_BLOCK_SIZE_BYTES`).
    if size == 0 {
      // 零长分配在 `Layout`/`alloc` API 下无合法形态（cpp nothrow new 亦不产
      // 可用块），以 `None` 交调用方按 bad_alloc 处理。
      return None;
    }
    let layout = match Layout::from_size_align(size, page_size()) {
      Ok(l) => l,
      Err(_) => return None,
    };
    // Safety: `Layout` 由「非零 size（0 已在上方早退）+ 2 的幂页对齐」构造，满足
    // `alloc` 的合法 layout 前置；返回块指针（成功）或 `None`（失败），与重建的
    // C++ `operator new(nothrow)` 契约一致——调用方 `TypedAllocator::append_block`
    // 对 `None` 按 bad_alloc 语义 panic。
    return unsafe { NonNull::new(alloc(layout)) };
  }

  // On Windows, VirtualAlloc results in 64K granularity allocations; on
  // Linux we must use mmap because using the regular heap results in
  // mprotect() fragmenting the page table and bumping into the 64K mmap
  // limit.
  #[cfg(target_os = "windows")]
  {
    use windows_sys::Win32::System::Memory::{
      MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc,
    };
    // Safety: 对 Win32 `VirtualAlloc` 的裸 FFI：`addr=null` 为 C ABI 要求的
    // 「由系统选址」形态（保留空指针），失败返回 NULL 经 `NonNull::new` 折为 `None`。
    unsafe {
      NonNull::new(
        VirtualAlloc(
          core::ptr::null(),
          size,
          MEM_RESERVE | MEM_COMMIT,
          PAGE_READWRITE,
        )
        .cast::<u8>(),
      )
    }
  }

  #[cfg(target_os = "freebsd")]
  {
    unsafe extern "C" {
      fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    }
    unsafe { NonNull::new(aligned_alloc(page_size(), size)) }
  }

  #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
  // Safety: 对 libc `mmap` 的裸 FFI：`addr=null` 为 C ABI 要求的「由内核选址」
  // 形态（保留空指针），`len` 经 `page_align` 向上取整到非零页粒度，
  // `PROT_READ|PROT_WRITE` + `MAP_PRIVATE|MAP_ANON` 配 `fd=-1, offset=0` 是匿名
  // 私有映射的标准合法参数组（与 C++ pagedAllocate 同形），全部为标量、无指针
  // 有效性负担；`MAP_FAILED(-1)` 归一为 `None` 供调用方按 bad_alloc 处理。
  unsafe {
    const PROT_READ: i32 = 0x1;
    const PROT_WRITE: i32 = 0x2;
    const MAP_PRIVATE: i32 = 0x02;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const MAP_ANON: i32 = 0x20;
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    const MAP_ANON: i32 = 0x1000;

    let result = mmap(
      null_mut(),
      page_align(size),
      PROT_READ | PROT_WRITE,
      MAP_PRIVATE | MAP_ANON,
      -1,
      0,
    );

    // mmap returns MAP_FAILED (-1) on error. Normalize it to `None` so the
    // caller (`append_block`) observes the failure.
    if result as isize == -1 {
      None
    } else {
      NonNull::new(result)
    }
  }
}
