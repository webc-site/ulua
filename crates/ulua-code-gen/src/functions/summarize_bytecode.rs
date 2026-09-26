use alloc::vec::Vec;

use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
use ulua_vm::{
  functions::{lua_a_toobject::lua_a_toobject, lua_is_lfunction::lua_is_lfunction},
  records::{lua_state::LuaState, proto::Proto},
  type_aliases::t_value::TValue,
};

use crate::{
  enums::code_gen_flags::CodeGenFlags, functions::gather_functions::gather_functions,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::function_bytecode_summary::FunctionBytecodeSummary,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn summarize_bytecode(
  l: *mut LuaState,
  idx: i32,
  nesting_limit: u32,
) -> Vec<FunctionBytecodeSummary> {
  // Safety: 契约保证 l 为存活 LuaState*、idx 为界内栈位；lua_is_lfunction/lua_a_toobject 依
  // Lua C-ABI 合法，clvalue!(func) 得存活 LClosure*，其 inner.l.p 为构造接线的非空 Proto*
  // （(*root).flags 同址 u8 直读），gather_functions/from_proto 依同一 proto 存活契约递归。
  // 下文各窄块统一简记「依契约」。

  // Safety: 依契约——栈位类型与 TValue 读数均走 Lua C-ABI。
  unsafe { CODEGEN_ASSERT!(lua_is_lfunction(l, idx) != 0) };
  let func: *const TValue = unsafe { lua_a_toobject(l, idx) };

  // Safety: 依契约——经 as_closure 将 func 视作存活 LClosure*，inner.l.p 为构造接线的非空 Proto*。
  let root: *mut Proto = unsafe { (*func).as_closure().inner.l.p };

  // 统计路径全程只读原型，一次派生共享引用即覆盖 flags 读取与逐函数汇总（review.md §2）。
  // Safety: `root` 依上契约存活，且本函数调用栈上不写 Proto。
  let root_ref = unsafe { &*root };

  // gather_functions 返回按 bytecodeid 索引的稀疏表（None 空槽），flatten 依序紧凑收集非空槽
  // Safety: 依契约——root 存活、flags 同址 u8 直读；callee 沿同一契约递归。
  let protos = unsafe {
    gather_functions(
      root,
      CodeGenFlags::CodeGenColdFunctions as u32,
      (root_ref.flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0,
    )
  };

  // 稀疏表长度为非空槽数的上界，直接作 capacity 提示（与原实现一致）
  let mut summaries: Vec<FunctionBytecodeSummary> = Vec::with_capacity(protos.len());

  for proto in protos.into_iter().flatten() {
    // Safety: 依契约——表元素皆源自以 root 为根、存活 proto 的收集结果，只读派生共享引用。
    let proto = unsafe { &*proto };
    summaries.push(FunctionBytecodeSummary::from_proto(proto, nesting_limit));
  }

  summaries
}
