use ulua_common::fflag;
use ulua_vm::{functions::lua_gettop::lua_gettop, records::lua_state};

use crate::{
  functions::{
    get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::{lua_check_args, lua_check_not_frozen, lua_check_tag},
  records::{
    type_function_never_type::TypeFunctionNeverType,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int setTableIndexer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1085`）。
pub unsafe fn set_table_indexer(l: *mut LuaState) -> i32 {
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 3, "type.setindexer: expected 3 arguments, but got {}");

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);

    lua_check_tag!(
      vm_l,
      tftt.is_null(),
      l,
      self_ty,
      "type.setindexer: expected self to be either a table, but got {} instead"
    );

    lua_check_not_frozen!(vm_l, self_ty, "type.setindexer");

    let key = get_type_user_data(l, 2);
    let value = get_type_user_data(l, 3);

    if !get_type_function_type_id::<TypeFunctionNeverType>(key).is_null() {
      (*tftt).indexer = None;
      return 0;
    }

    (*tftt).indexer = Some(TypeFunctionTableIndexer::new(key, value));
    0
  }
}
