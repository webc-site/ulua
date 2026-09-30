use crate::{
  functions::type_function_runtime_entries::set_table_prop_rw, type_aliases::lua_state::LuaState,
};

/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int setReadTableProp(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:913`）。
pub unsafe fn set_read_table_prop(l: *mut LuaState) -> i32 {
  // Safety: 前置条件即本函数 # Safety 段的 VM 回调契约，转交共享实现；
  // `read = true` 改写 prop 的 `read_ty` 一侧。
  unsafe { set_table_prop_rw(l, "type.setreadproperty", true) }
}
