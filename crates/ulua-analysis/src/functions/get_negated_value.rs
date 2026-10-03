

/// 对应 C++ 原生 `static int getNegatedValue(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:778`）。
use ulua_vm::records::lua_state;
use crate::{functions::{alloc_type_user_data::alloc_type_user_data, get_tag::get_tag, get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data, throw_type_error::throw_type_error}, macros::lua_check_args, records::type_function_negation_type::TypeFunctionNegationType, type_aliases::{type_function_type_id::TypeFunctionTypeId}};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn get_negated_value(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活，`l as *mut lua_state::LuaState` 为同址重解释。
  // `tfnt` 由 class-index 下转取得，仅在 `!is_null()` 守卫分支解引用 `(*tfnt).type_id`，其指向
  // arena 存活的类型对象（地址不移动）。else 分支的 `throw_type_error` 返回 `!` 不返回。单线程串行，
  // 无并发别名。
  unsafe {
    lua_check_args!(l, != 1, "type.inner: expected 1 argument, but got {}");

    let self_ty: TypeFunctionTypeId = get_type_user_data(&mut *l, 1);
    let tfnt = get_type_function_type_id::<TypeFunctionNegationType>(self_ty);

    if !tfnt.is_null() {
      alloc_type_user_data(&mut *l, (*(*tfnt).type_id).type_variant.clone(), false);
    } else {
      throw_type_error(
        l.as_mut_ptr(),
        format_args!(
          "type.inner: cannot call inner method on non-negation type: `{}` type",
          get_tag(&mut *l, self_ty)
        ),
      );
    }

    1
  }
}
