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

pub fn allocate_pages_impl(size: usize) -> *mut u8 {
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::Memory::{
      MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc,
    };

    // Safety: VirtualAlloc 为 Win32 C ABI——addr=null 由内核选址、size>0 且断言页对齐、
    // MEM_RESERVE|MEM_COMMIT + PAGE_READWRITE 合法组合；失败返回 null 由调用方判空，
    // 成功即指向存活的私有提交区。
    unsafe {
      VirtualAlloc(
        core::ptr::null::<c_void>(),
        size,
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
      ) as *mut u8
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

    // Safety: 本分支为 mmap C ABI 调用，实参皆常量：addr=null（内核选址）、len=size>0 且断言
    // 页对齐、prot=READ|WRITE、flags=PRIVATE|ANON(|JIT)、fd=-1、offset=0，是合法匿名私有映射。
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

    // 返回 MAP_FAILED(-1) 已折叠为 null_mut()，故返回值或为空、或指向存活的映射区。
    if result as isize == -1 {
      null_mut()
    } else {
      result.cast()
    }
  }
}
