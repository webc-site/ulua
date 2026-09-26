#[cfg(target_os = "windows")]
use core::ffi::c_void;

#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Memory::{PAGE_EXECUTE_READ, VirtualProtect};

use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn make_pages_executable(mem: *mut u8, size: usize) -> bool {
  CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(target_os = "windows")]
  {
    let mut old_protect: u32 = 0;
    unsafe {
      VirtualProtect(
        mem as *const c_void,
        size,
        PAGE_EXECUTE_READ,
        &mut old_protect,
      ) != 0
    }
  }

  #[cfg(not(target_os = "windows"))]
  {
    // Safety: 进入本分支前已断言 mem 页对齐、size 为整页数, 与 make_pages_executable_mut
    // 的契约一致, 直接把同一已验证的活映射区间转交被调 unsafe fn, 无额外解引用。
    unsafe { make_pages_executable_mut(mem, size) }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn make_pages_executable_mut(mem: *mut u8, size: usize) -> bool {
  // 断言为纯 safe 计算，置于 unsafe 之外；unsafe 只收窄到真正的 FFI 调用处。
  CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
  {
    use core::ffi::c_void;

    unsafe extern "C" {
      fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
    }

    const PROT_READ: i32 = 0x1;
    const PROT_EXEC: i32 = 0x4;

    // Safety: mem/size 为分配器配对产出的存活映射区，上方断言保证 mem 页对齐、
    // size 为整页数；mprotect 为 POSIX C ABI，仅改该区保护位、不解引用内容。
    unsafe { mprotect(mem as *mut c_void, size, PROT_READ | PROT_EXEC) == 0 }
  }
  #[cfg(target_os = "windows")]
  {
    // Windows 路径与 base 实现逐行相同，转发消除 VirtualProtect 重复体
    // Safety: mem/size 为上方断言已验证的同一段活映射区间，转交同契约的 unsafe fn。
    unsafe {
      crate::functions::make_pages_executable_code_allocator::make_pages_executable(mem, size)
    }
  }
  #[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "windows"
  )))]
  {
    // wasm32 等目标无可执行页保护概念，恒为 false（与不登记 unwind 同理）。
    let _ = (mem, size);
    false
  }
}
