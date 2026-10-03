

/// 对应 C++ 原生 `static int setTableIndexer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1085`）。
use crate::{functions::{get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data}, macros::{lua_check_args, lua_check_not_frozen, lua_check_tag}, records::{type_function_never_type::TypeFunctionNeverType, type_function_table_indexer::TypeFunctionTableIndexer, type_function_table_type::TypeFunctionTableType}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn set_table_indexer(l: &mut LuaState) -> i32 {
  unsafe {
    lua_check_args!(l, != 3, "type.setindexer: expected 3 arguments, but got {}");

    let self_ty = get_type_user_data(&mut *l, 1);
    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);

    lua_check_tag!(
      l,
      tftt.is_null(),
      self_ty,
      "type.setindexer: expected self to be either a table, but got {} instead"
    );

    lua_check_not_frozen!(l, self_ty, "type.setindexer");

    let key = get_type_user_data(&mut *l, 2);
    let value = get_type_user_data(&mut *l, 3);

    if !get_type_function_type_id::<TypeFunctionNeverType>(key).is_null() {
      (*tftt).indexer = None;
      return 0;
    }

    (*tftt).indexer = Some(TypeFunctionTableIndexer::new(key, value));
    0
  }
}
