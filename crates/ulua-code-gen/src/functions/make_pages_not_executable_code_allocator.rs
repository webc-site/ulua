#[cfg(any(target_os = "linux", target_os = "macos"))]
use core::ffi::c_void;

use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

pub fn make_pages_not_executable(mem: *mut u8, size: usize) -> bool {
  CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  make_pages_not_executable_mut(mem, size)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
unsafe extern "C" {
  fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
}

pub fn make_pages_not_executable_mut(mem: *mut u8, size: usize) -> bool {
  CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(any(target_os = "linux", target_os = "macos"))]
  {
    const PROT_READ: i32 = 0x1;
    const PROT_WRITE: i32 = 0x2;

    // Safety: mem 断言为页对齐、size 断言等于页对齐值；mprotect 为 POSIX C ABI，addr 非空
    // 且页对齐、len 为其映射区内页对齐长度、prot=READ|WRITE 合法，返回码==0 折叠为布尔结果。
    unsafe { mprotect(mem as *mut c_void, size, PROT_READ | PROT_WRITE) == 0 }
  }

  // Windows 原本是不实现的 `false` stub，导致 CodeAllocator::deallocate（它
  // 对结果做 CODEGEN_ASSERT!）在 Windows 上每次释放代码块都以 0x80000003
  // abort。此处对齐可执行路径的 VirtualProtect，把保护属性
  // 翻回 PAGE_READWRITE。
  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::Memory::{PAGE_READWRITE, VirtualProtect};

    let mut old_protect: u32 = 0;
    // Safety: mem/size 入口断言页对齐且 mem 为分配器映射区内地址；VirtualProtect 为
    // Win32 C ABI，PAGE_READWRITE 为合法新保护位，old_protect 为存活 &mut 输出。
    unsafe {
      VirtualProtect(
        mem as *const c_void,
        size,
        PAGE_READWRITE,
        &mut old_protect as *mut u32,
      ) != 0
    }
  }

  #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
  {
    false
  }
}
