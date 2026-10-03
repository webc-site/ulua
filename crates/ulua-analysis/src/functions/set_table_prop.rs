

use crate::functions::get_tag::get_tag;
use ulua_common::fflag;
/// 对应 C++ 原生 `static int setTableProp(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:876`）。
use crate::{functions::{get_mutable_type_function_runtime::get_mutable_type_function_type_id, get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data, throw_type_error::throw_type_error}, macros::{lua_check_args, lua_check_not_frozen, lua_check_tag}, records::{type_function_property::TypeFunctionProperty, type_function_singleton_type::TypeFunctionSingletonType, type_function_table_type::TypeFunctionTableType}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn set_table_prop(l: &mut LuaState) -> i32 {
  unsafe {
    lua_check_args!(
      argument_count = l,
      2..=3,
      "type.setproperty: expected 2-3 arguments, but got {}"
    );

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);
    lua_check_tag!(
      l,
      tftt.is_null(),
      self_ty,
      "type.setproperty: expected self to be a table, but got {} instead"
    );

    lua_check_not_frozen!(l, self_ty, "type.setproperty");

    let key = get_type_user_data(l, 2);
    let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(key);
    lua_check_tag!(
      l,
      tfst.is_null(),
      key,
      "type.setproperty: expected to be given a singleton type, but got {} instead"
    );

    let tfsst = (*tfst).variant.get_if_1();
    lua_check_tag!(
      l,
      tfsst.is_none(),
      key,
      "type.setproperty: expected to be given a string singleton type, but got {} instead"
    );

    // `throw_type_error` 静态类型 `-> !`：is_none 分支必不返回，块后 Some 由其蕴含。
    let key_name = tfsst
      .expect("上方 throw_type_error(-> !) 已拦截 None 分支")
      .value
      .clone();

    if argument_count == 2 || l.is_nil(3) {
      (*tftt).props.remove(&key_name);
      return 0;
    }

    let value = get_type_user_data(l, 3);
    (*tftt).props.insert(
      key_name,
      TypeFunctionProperty {
        read_ty: Some(value),
        write_ty: Some(value),
      },
    );

    0
  }
}
