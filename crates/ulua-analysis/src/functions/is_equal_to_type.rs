use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_pushboolean::lua_pushboolean},
  records::lua_state,
};

use crate::{
  functions::{get_type_user_data::get_type_user_data, throw_type_error::throw_type_error},
  macros::lua_check_args,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int isEqualToType(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1884`）。
pub unsafe fn is_equal_to_type(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活，`l as *mut lua_state::LuaState` 为同址重解释。
  // 参数个数不符时 `throw_type_error` 返回 `!` 不返回；`self_ty`/`arg` 来自 `get_type_user_data`，
  // 该函数要么返回指向存活 tagged userdata 的非空 TypeFunctionTypeId、要么抛类型错误（返回 `!`）。
  // 故 `*self_ty == *arg` 解引用合法，二者在本次调用内存活。单线程串行，无并发别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 2, "expected 2 arguments, but got {}");

    let self_ty = get_type_user_data(l, 1);
    let arg = get_type_user_data(l, 2);

    lua_pushboolean(vm_l, (*self_ty == *arg) as i32);
    1
  }
}
