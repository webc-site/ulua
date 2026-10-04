use ulua_common::fflag;
use ulua_vm::records::lua_state::LuaState;

use crate::functions::{get_tag::get_tag, throw_type_error::throw_type_error};
/// 对应 C++ 原生 `static int setTableIndexer(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1085`）。
use crate::{
  functions::{
    get_mutable_type_function_runtime::get_mutable_type_function_type_id,
    get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data,
  },
  macros::{lua_check_args, lua_check_not_frozen, lua_check_tag},
  records::{
    type_function_never_type::TypeFunctionNeverType,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType,
  },
};
pub(crate) fn set_table_indexer(l: &mut LuaState) -> i32 {
  unsafe {
    lua_check_args!(l, != 3, "type.setindexer: expected 3 arguments, but got {}");

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);

    lua_check_tag!(
      l,
      tftt.is_none(),
      self_ty,
      "type.setindexer: expected self to be either a table, but got {} instead"
    );

    lua_check_not_frozen!(l, self_ty, "type.setindexer");

    // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
    let tftt = tftt.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");

    let key = get_type_user_data(l, 2);
    let value = get_type_user_data(l, 3);

    if get_type_function_type_id::<TypeFunctionNeverType>(key).is_some() {
      tftt.indexer = None;
      return 0;
    }

    tftt.indexer = Some(TypeFunctionTableIndexer::new(key, value));
    0
  }
}
