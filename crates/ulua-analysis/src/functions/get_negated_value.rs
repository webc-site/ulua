use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::type_function_negation_type::TypeFunctionNegationType,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};
pub(crate) fn get_negated_value(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活（经 `c_thunk!` 蹦床重建为独占 `&mut`）。
  // `tfnt` 由 class-index 下转取得，仅命中 Some 分支才读取 `type_id`，其指向 arena 存活的
  // 类型对象（地址不移动）。else 分支的 `throw_type_error` 返回 `!` 不返回。单线程串行，
  // 无并发别名。
  unsafe {
    lua_check_args!(l, != 1, "type.inner: expected 1 argument, but got {}");

    let self_ty: TypeFunctionTypeId = get_type_user_data(l, 1);

    if let Some(tfnt) = get_type_function_type_id::<TypeFunctionNegationType>(self_ty) {
      alloc_type_user_data(l, (*tfnt.type_id).type_variant.clone(), false);
    } else {
      let tag = get_tag(l, self_ty);
      throw_type_error(
        l,
        format_args!(
          "type.inner: cannot call inner method on non-negation type: `{}` type",
          tag
        ),
      );
    }

    1
  }
}
