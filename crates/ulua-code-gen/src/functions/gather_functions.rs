use alloc::vec::Vec;

use ulua_vm::records::proto::Proto;

use crate::functions::gather_functions_helper::gather_functions_helper;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn gather_functions(
  results: &mut Vec<*mut Proto>,
  root: *mut Proto,
  flags: u32,
  has_native_functions: bool,
) {
  unsafe { gather_functions_helper(results, root, flags, has_native_functions, true) };
}
