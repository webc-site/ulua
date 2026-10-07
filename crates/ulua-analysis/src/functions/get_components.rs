/// 对应 C++ 原生 `static int getComponents(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:717`）。
use ulua_vm::functions::{lua_createtable::lua_createtable, lua_rawseti::lua_rawseti};
use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    alloc_type_user_data::alloc_type_user_data, get_tag::get_tag,
    get_type_function_runtime::get_type_function_type_id, get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::{
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_union_type::TypeFunctionUnionType,
  },
};
pub(crate) fn get_components(l: &mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时的 C 调用约定传入，非空且在整个调用内存活；
  // `l` 由 `c_thunk!` 蹦床重建为独占 `&mut`。`get_type_user_data`
  // 命中 Some 的返回类型才读取 `components`：`get_type_function_type_id` 按 RTTI
  // class-index 下转，命中即类型正确；`*component`(arena 存活 TypeId、地址不移动) 读取
  // `type_variant` 合法；未命中分支的
  // `throw_type_error` 返回 `!`（longjmp 不返回）。单线程串行执行，VM 栈操作无并发别名。
  unsafe {
    lua_check_args!(l, != 1, "type.components: expected 1 argument, but got {}");

    let self_ty = get_type_user_data(l, 1);

    if let Some(tfut) = get_type_function_type_id::<TypeFunctionUnionType>(self_ty) {
      let components = &tfut.components;

      lua_createtable(l.as_mut_ptr(), components.len() as i32, 0);
      for (i, &component) in components.iter().enumerate() {
        alloc_type_user_data(l, (*component).type_variant.clone(), false);
        lua_rawseti(l, -2, i as i32 + 1);
      }

      return 1;
    }

    if let Some(tfit) = get_type_function_type_id::<TypeFunctionIntersectionType>(self_ty) {
      let components = &tfit.components;

      lua_createtable(l.as_mut_ptr(), components.len() as i32, 0);
      for (i, &component) in components.iter().enumerate() {
        alloc_type_user_data(l, (*component).type_variant.clone(), false);
        lua_rawseti(l, -2, i as i32 + 1);
      }

      return 1;
    }

    let tag = get_tag(l, self_ty);
    throw_type_error(
      l,
      format_args!("type.components: cannot call components of `{}` type", tag),
    );
  }
}
