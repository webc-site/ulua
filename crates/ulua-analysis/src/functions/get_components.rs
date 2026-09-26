use ulua_vm::{
  functions::{lua_createtable::lua_createtable, lua_gettop::lua_gettop, lua_rawseti::lua_rawseti},
  records::lua_state,
};

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
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int getComponents(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:717`）。
pub unsafe fn get_components(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时的 C 调用约定传入，非空且在整个调用内存活；
  // `l as *mut lua_state::LuaState` 是同一 VM 状态的重解释（同一地址）。`get_type_user_data`
  // 返回的类型函数指针经 `is_null` 判定后才解引用：`get_type_function_type_id` 按 RTTI
  // class-index 下转，命中即类型正确，故 `(*tfut).components`/`(*tfit).components` 及其中
  // `*component`(arena 存活 TypeId、地址不移动) 读取 `type_variant` 合法；未命中分支的
  // `throw_type_error` 返回 `!`（longjmp 不返回）。单线程串行执行，VM 栈操作无并发别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 1, "type.components: expected 1 argument, but got {}");

    let self_ty = get_type_user_data(l, 1);

    let tfut = get_type_function_type_id::<TypeFunctionUnionType>(self_ty);
    if !tfut.is_null() {
      let components = &(*tfut).components;

      lua_createtable(vm_l, components.len() as i32, 0);
      for (i, &component) in components.iter().enumerate() {
        alloc_type_user_data(l, (*component).type_variant.clone(), false);
        lua_rawseti(vm_l, -2, i as i32 + 1);
      }

      return 1;
    }

    let tfit = get_type_function_type_id::<TypeFunctionIntersectionType>(self_ty);
    if !tfit.is_null() {
      let components = &(*tfit).components;

      lua_createtable(vm_l, components.len() as i32, 0);
      for (i, &component) in components.iter().enumerate() {
        alloc_type_user_data(l, (*component).type_variant.clone(), false);
        lua_rawseti(vm_l, -2, i as i32 + 1);
      }

      return 1;
    }

    let tag = get_tag(l, self_ty);
    throw_type_error(
      vm_l,
      format_args!("type.components: cannot call components of `{}` type", tag),
    );
  }
}
