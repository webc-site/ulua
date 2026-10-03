

use crate::functions::get_tag::get_tag;
use crate::{functions::{get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data, throw_type_error::throw_type_error}, macros::lua_check_tag, records::type_function_generic_type::TypeFunctionGenericType};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn get_generic_is_pack(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活，`l as *mut lua_state::LuaState`
  // 为同一地址的重解释。`get_type_function_type_id::<TypeFunctionGenericType>` 按 class-index
  // 下转，其 `is_null()` 分支内 `throw_type_error` 返回 `!`（抛错不返回），故其后 `(*tfgt).is_pack`
  // 解引用合法；类型函数数据存活于本次调用。单线程串行，无并发别名。
  unsafe {
    let self_ty = get_type_user_data(&mut *l, 1);
    let tfgt = get_type_function_type_id::<TypeFunctionGenericType>(self_ty);

    lua_check_tag!(
      l,
      tfgt.is_null(),
      self_ty,
      "type.ispack: expected self to be a generic, but got {} instead"
    );

    l.push_boolean((*tfgt).is_pack);
    1
  }
}
