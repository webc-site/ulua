use ulua_vm::records::lua_state::LuaState;

/// 对应 C++ 原生 `static int createNegation(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:761`）。
use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::{
    type_function_function_type::TypeFunctionFunctionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};
pub(crate) fn create_negation(l: &mut LuaState) -> i32 {
  // Safety: l 为 VM 调注册闭包传入的存活 lua_State；实参个数先校验（错误经 throw_type_error 收口
  // +匹配实参的 throw_type_error 终止）；get_type_user_data 非 type 实参抛错，arg 指向
  // type_arena 存活节点；两个 get_type_function_type_id 仅做 is_some 判别、未命中不解引用；
  // negation 存的是同 arena 稳定裸指针，alloc_type_user_data 前置同族约定满足。
  unsafe {
    lua_check_args!(l, != 1, "types.negationof: expected 1 argument, but got {}");

    let arg = get_type_user_data(l, 1);

    if get_type_function_type_id::<TypeFunctionTableType>(arg).is_some()
      || get_type_function_type_id::<TypeFunctionFunctionType>(arg).is_some()
    {
      let tag = get_tag(l, arg);
      throw_type_error(
        l,
        format_args!(
          "types.negationof: cannot perform negation on `{}` type",
          tag
        ),
      );
    }

    let negation = TypeFunctionNegationType { type_id: arg };
    alloc_type_user_data(l, TypeFunctionTypeVariant::Negation(negation), false);

    1
  }
}
