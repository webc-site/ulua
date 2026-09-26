use ulua_common::fflag;
use ulua_vm::{functions::lua_gettop::lua_gettop, records::lua_state};

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, deep_clone::deep_clone,
    get_type_function_runtime::get_type_function_runtime, get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int deepCopy(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1865`）。
pub unsafe fn deep_copy(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 1, "types.copy: expected 1 arguments, but got {}");

    let arg = get_type_user_data(l, 1);
    let runtime = get_type_function_runtime(l);
    let copy = deep_clone(runtime, arg);

    if fflag::LuauTypeFunctionRobustness.get() && copy.is_null() {
      throw_type_error(
        vm_l,
        format_args!("types.copy: complexity limit reached during type copy"),
      );
    }

    alloc_type_user_data(l, (*copy).type_variant.clone(), false);
    1
  }
}
