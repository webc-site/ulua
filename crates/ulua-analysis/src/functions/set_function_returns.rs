use ulua_common::fflag;
use ulua_vm::{functions::lua_gettop::lua_gettop, records::lua_state};

use crate::{
  functions::{
    create_function::get_type_pack_runtime,
    get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_user_data::get_type_user_data, throw_type_error::throw_type_error,
  },
  macros::{lua_check_args, lua_check_not_frozen, lua_check_tag},
  records::type_function_function_type::TypeFunctionFunctionType,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int setFunctionReturns(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1402`）。
pub unsafe fn set_function_returns(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(
      vm_l,
      2..=3,
      "type.setreturns: expected 1-3 arguments, but got {}"
    );

    let self_ty = get_type_user_data(l, 1);
    let tfft = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
    lua_check_tag!(
      vm_l,
      tfft.is_null(),
      l,
      self_ty,
      "type.setreturns: expected self to be a function, but got {} instead"
    );

    lua_check_not_frozen!(vm_l, self_ty, "type.setreturns");

    (*tfft).ret_types = get_type_pack_runtime(l, 2, 3);

    0
  }
}
