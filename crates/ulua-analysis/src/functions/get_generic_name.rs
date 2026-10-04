use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    get_tag::get_tag, get_type_function_runtime::get_type_function_type_id,
    get_type_user_data::get_type_user_data, push_string::push_string,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_tag,
  records::type_function_generic_type::TypeFunctionGenericType,
};
pub(crate) fn get_generic_name(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活（经 `c_thunk!` 蹦床重建为独占 `&mut`）。
  // `lua_check_tag` 的 `is_none()` 分支内 `throw_type_error` 返回 `!`（抛错不返回），其后
  // `expect` 取回的引用由该 Some 证明蕴含；`push_string` 按长度读取该 String，借用随
  // arena 节点在本次调用内存活。单线程串行，无并发别名。
  unsafe {
    let self_ty = get_type_user_data(l, 1);

    let tfgt = get_type_function_type_id::<TypeFunctionGenericType>(self_ty);
    lua_check_tag!(
      l,
      tfgt.is_none(),
      self_ty,
      "type.name: expected self to be a generic, but got {} instead"
    );

    let tfgt = tfgt.expect("上方 is_none 分支经 throw_type_error(-> !) 早退，至此必为 Some");
    if tfgt.is_named {
      let n = &tfgt.name;
      push_string(l, n);
    } else {
      l.push_nil();
    }

    1
  }
}
