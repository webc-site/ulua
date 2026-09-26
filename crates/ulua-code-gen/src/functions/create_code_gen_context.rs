use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::initialize_execution_callbacks::initialize_execution_callbacks,
  records::shared_code_gen_context::SharedCodeGenContext,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create(l: *mut LuaState, code_gen_context: *mut SharedCodeGenContext) {
  // Safety: code_gen_context 为存活 SharedCodeGenContext*，其 `base` 首字段与
  // BaseCodeGenContext #[repr(C)] 同址，派生 &mut 合法。
  let base_context = unsafe { &mut (*code_gen_context).base };
  // Safety: base_context 依 SharedCodeGenContext→BaseCodeGenContext 的 #[repr(C)] 首字段
  // 同址关系取 &mut，`as *mut _` 上转型安全；initialize_execution_callbacks 按其契约消费
  // l 与该 base 指针，转发未放宽裸指针前提。
  unsafe { initialize_execution_callbacks(l, base_context as *mut _) };
}
