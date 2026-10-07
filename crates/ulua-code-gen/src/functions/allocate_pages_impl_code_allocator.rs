use core::ptr::NonNull;
#[cfg(not(target_os = "windows"))]
use core::{ffi::c_void, ptr::null_mut};

use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

#[cfg(not(target_os = "windows"))]
unsafe extern "C" {
  fn mmap(
    addr: *mut c_void,
    len: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: isize,
  ) -> *mut c_void;
}

/// 映射 `size`（须页对齐）字节可写私有内存。`None` 即「映射失败」——cpp 的
/// 「返回 null 由调用方判空」在 Rust 侧是类型化的缺席语义，不再是裸指针哨兵
/// （review.md §2）。
pub fn allocate_pages_impl(size: usize) -> Option<NonNull<u8>> {
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::Memory::{
      MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc,
    };

    // Safety: VirtualAlloc 为 Win32 C ABI——addr=null 由内核选址、size>0 且断言页对齐、
    // MEM_RESERVE|MEM_COMMIT + PAGE_READWRITE 合法组合；失败返回 null 折叠为 `None`，
    // 成功即指向存活的私有提交区，`NonNull::new` 保非空。
    unsafe {
      NonNull::new(VirtualAlloc(
        core::ptr::null::<c_void>(),
        size,
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
      ) as *mut u8)
    }
  }

  #[cfg(not(target_os = "windows"))]
  {
    const PROT_READ: i32 = 0x1;
    const PROT_WRITE: i32 = 0x2;
    const MAP_PRIVATE: i32 = 0x02;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const MAP_ANON: i32 = 0x20;
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    const MAP_ANON: i32 = 0x1000;
    // 其余目标（如 wasm32-unknown-unknown）无 mmap 语义；本函数仅在
    // is_supported 平台上被调用，此处仅为满足编译
    #[cfg(not(any(
      target_os = "linux",
      target_os = "android",
      target_os = "macos",
      target_os = "ios",
      target_os = "freebsd"
    )))]
    const MAP_ANON: i32 = 0;
    #[cfg(target_os = "macos")]
    const MAP_JIT: i32 = 0x0800;
    #[cfg(not(target_os = "macos"))]
    const MAP_JIT: i32 = 0;

    // Safety: 本分支为 mmap C ABI 调用，实参皆常量：addr=null（内核选址，真 FFI 边界的
    // `null_mut()`）、len=size>0 且断言页对齐、prot=READ|WRITE、flags=PRIVATE|ANON(|JIT)、
    // fd=-1、offset=0，是合法匿名私有映射。
    let result = unsafe {
      mmap(
        null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANON | MAP_JIT,
        -1,
        0,
      )
    };

    // MAP_FAILED(-1) 不是有效地址，先折叠为 `None`；其余按 mmap 非空返回值收编 `NonNull`。
    if result as isize == -1 {
      None
    } else {
      NonNull::new(result.cast::<u8>())
    }
  }
}
