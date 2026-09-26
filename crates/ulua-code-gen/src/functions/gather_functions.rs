use alloc::vec::Vec;

use ulua_vm::records::proto::Proto;

use crate::functions::gather_functions_helper::gather_functions_helper;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn gather_functions(
  root: *mut Proto,
  flags: u32,
  has_native_functions: bool,
) -> Vec<Option<*mut Proto>> {
  // 稀疏表：按 bytecodeid 索引，`None` 为空槽（cpp 的 nullptr 哨兵），元素指针为 C-ABI 裸指针。
  let mut results: Vec<Option<*mut Proto>> = Vec::new();
  // Safety: 本 unsafe fn 的 `# Safety` 契约保证 root 为存活 Proto*、results 为其借用，
  // 与被转发的 gather_functions_helper 前置条件一致（递归遍历期间 protos 存活），
  // 转发未收紧或放宽裸指针契约。
  unsafe { gather_functions_helper(&mut results, root, flags, has_native_functions, true) };
  results
}
