use alloc::vec::Vec;

use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
use ulua_vm::records::proto::Proto;

use crate::{
  enums::code_gen_flags::CodeGenFlags,
  functions::proto_views::child_proto_refs,
};

/// 递归收集（累加器 `results` 为跨递归复用的输出缓冲，非出参）。
///
/// `results` 按 `bytecodeid` 索引的稀疏表：`None` 表示该槽尚未收集（对齐 cpp 的
/// `nullptr` 空槽语义），元素借用随 `root` 的存活契约（整段编译会话内由 VM 持有，
/// 见 `proto_views` 模块文档）。
pub(crate) fn gather_functions_helper<'a>(
  results: &mut Vec<Option<&'a Proto>>,
  proto: &'a Proto,
  flags: u32,
  has_native_functions: bool,
  root: bool,
) {
  let id = proto.bytecodeid as usize;
  if results.len() <= id {
    results.resize(id + 1, None);
  }

  if results[id].is_some() {
    return;
  }

  let should_gather = if has_native_functions {
    !root && LuauProtoFlag::LPF_NATIVE_FUNCTION.is_set(proto.flags)
  } else {
    !LuauProtoFlag::LPF_NATIVE_COLD.is_set(proto.flags)
      || CodeGenFlags::CodeGenColdFunctions.is_set(flags)
  };

  if should_gather {
    results[id] = Some(proto);
  }

  // 子原型按 `proto_views::child_proto_refs` 的安全引用视图迭代：`sizep == 0` 时
  // 基址允许为 null（cpp 空循环语义），视图统一折成空切片，无需再手工判空。
  for child_proto in child_proto_refs(proto) {
    gather_functions_helper(results, child_proto, flags, has_native_functions, false);
  }
}
