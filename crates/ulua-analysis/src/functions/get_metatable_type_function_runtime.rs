use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_pushnil::lua_pushnil},
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
    type_function_extern_type::TypeFunctionExternType,
    type_function_table_type::TypeFunctionTableType,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int getMetatable(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1786`）。
pub unsafe fn get_metatable(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活，`l as *mut lua_state::LuaState` 为同址重解释。
  // `tfmt`/`tfct` 按 class-index 下转，仅在 `!is_null()` 守卫后解引用；`(*..).metatable` 为
  // Option<TypeId>，`if let Some(metatable)` 命中后 `(*metatable)` 指向 arena 存活 TypeVar
  // （bump 分配、地址不移动），读取其 type_variant 合法。末尾 `throw_type_error` 分支返回 `!` 不返回。
  // 单线程串行执行，无并发别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 1, "type.metatable: expected 1 arguments, but got {}");

    let self_ty = get_type_user_data(l, 1);

    let tfmt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if !tfmt.is_null() {
      if let Some(metatable) = (*tfmt).metatable {
        alloc_type_user_data(l, (*metatable).type_variant.clone(), false);
      } else {
        lua_pushnil(vm_l);
      }
      return 1;
    }

    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if !tfct.is_null() {
      if let Some(metatable) = (*tfct).metatable {
        alloc_type_user_data(l, (*metatable).type_variant.clone(), false);
      } else {
        lua_pushnil(vm_l);
      }
      return 1;
    }

    throw_type_error(
      vm_l,
      format_args!(
        "type.metatable: expected self to be a table or class, but got {} instead",
        get_tag(l, self_ty)
      ),
    );
  }
}
