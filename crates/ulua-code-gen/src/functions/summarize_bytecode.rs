use alloc::vec::Vec;

use ulua_common::enums::luau_proto_flag::LuauProtoFlag;
use ulua_vm::{
  functions::{lua_a_toobject::luaA_toobject, lua_is_lfunction::lua_is_lfunction},
  macros::clvalue::clvalue,
  records::proto::Proto,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

use crate::{
  enums::code_gen_flags::CodeGenFlags, functions::gather_functions::gather_functions,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::function_bytecode_summary::FunctionBytecodeSummary,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn summarize_bytecode(
  l: *mut lua_State,
  idx: i32,
  nesting_limit: u32,
) -> Vec<FunctionBytecodeSummary> {
  unsafe {
    CODEGEN_ASSERT!(lua_is_lfunction(l, idx) != 0);
    let func: *const TValue = luaA_toobject(l, idx);

    let cl = clvalue!(func);
    let root: *mut Proto = (*cl).inner.l.p;

    let mut protos: Vec<*mut Proto> = Vec::new();
    gather_functions(
      &mut protos,
      root,
      CodeGenFlags::CodeGenColdFunctions as u32,
      ((*root).flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0,
    );

    let mut summaries: Vec<FunctionBytecodeSummary> = Vec::with_capacity(protos.len());

    for proto in protos {
      if !proto.is_null() {
        summaries.push(FunctionBytecodeSummary::from_proto(proto, nesting_limit));
      }
    }

    summaries
  }
}
