use alloc::vec::Vec;

use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
use ulua_vm::records::proto::Proto;

use crate::{enums::code_gen_flags::CodeGenFlags, functions::proto_views::child_protos};

/// 递归收集（累加器 `results` 为跨递归复用的输出缓冲，非出参）。
///
/// `results` 按 `bytecodeid` 索引的稀疏表：`None` 表示该槽尚未收集（对齐 cpp 的
/// `nullptr` 空槽语义），元素指针本身为跨 crate 的 C-ABI `*mut Proto`。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn gather_functions_helper(
  results: &mut Vec<Option<*mut Proto>>,
  proto: *mut Proto,
  flags: u32,
  has_native_functions: bool,
  root: bool,
) {
  // Safety: `proto` 依契约指向存活 `Proto`——顶层由调用方传入、递归参数取自下方子原型切片，
  // 皆 VM 拥有且在 code-gen 期间不被回收；此处仅派生只读引用 `&*proto` 读字段，无 &mut 别名。
  let proto_ref = unsafe { &*proto };

  if results.len() <= proto_ref.bytecodeid as usize {
    results.resize(proto_ref.bytecodeid as usize + 1, None);
  }

  if results[proto_ref.bytecodeid as usize].is_some() {
    return;
  }

  let should_gather = if has_native_functions {
    !root && LuauProtoFlag::LPF_NATIVE_FUNCTION.is_set(proto_ref.flags)
  } else {
    !LuauProtoFlag::LPF_NATIVE_COLD.is_set(proto_ref.flags)
      || CodeGenFlags::CodeGenColdFunctions.is_set(flags)
  };

  if should_gather {
    results[proto_ref.bytecodeid as usize] = Some(proto);
  }

  // 子原型数组按 `proto_views::child_protos` 的安全切片视图迭代：`sizep == 0` 时基址允许为
  // null（cpp 空循环语义），视图统一折成空切片，无需再手工判空。
  for &child_proto in child_protos(proto_ref) {
    // Safety: `child_proto` 取自上方合法子原型视图，指向存活 `Proto`；`results` 的可变借用
    // 在递归内独占使用（视图仅产出 Copy 的裸指针，不与 results 借用冲突），递归维持同契约。
    unsafe { gather_functions_helper(results, child_proto, flags, has_native_functions, false) };
  }
}
