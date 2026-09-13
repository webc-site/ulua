use ulua_vm::records::lua_state::lua_State;

use crate::{
  functions::initialize_execution_callbacks::initialize_execution_callbacks,
  records::shared_code_gen_context::SharedCodeGenContext,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create(l: *mut lua_State, code_gen_context: *mut SharedCodeGenContext) {
  // SAFETY: code_gen_context is a pointer to a SharedCodeGenContext,
  // which inherits from BaseCodeGenContext. The initialize_execution_callbacks
  // function expects a pointer to the base class.
  let base_context = unsafe { &mut (*code_gen_context).base };
  unsafe { initialize_execution_callbacks(l, base_context as *mut _) };
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_lua_state_shared_code_gen_context(
  l: *mut lua_State,
  code_gen_context: *mut SharedCodeGenContext,
) {
  unsafe { create(l, code_gen_context) };
}
