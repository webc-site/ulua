use ulua_vm::{functions::lua_pushboolean::lua_pushboolean, records::lua_state};

use crate::{
  functions::{
    get_tag::get_tag, get_type_function_runtime::get_type_function_type_id,
    get_type_user_data::get_type_user_data, throw_type_error::throw_type_error,
  },
  macros::lua_check_tag,
  records::type_function_generic_type::TypeFunctionGenericType,
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int getGenericIsPack(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1555`）。
pub unsafe fn get_generic_is_pack(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 按类型函数运行时约定传入并全程存活，`l as *mut lua_state::LuaState`
  // 为同一地址的重解释。`get_type_function_type_id::<TypeFunctionGenericType>` 按 class-index
  // 下转，其 `is_null()` 分支内 `throw_type_error` 返回 `!`（抛错不返回），故其后 `(*tfgt).is_pack`
  // 解引用合法；类型函数数据存活于本次调用。单线程串行，无并发别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let self_ty = get_type_user_data(l, 1);
    let tfgt = get_type_function_type_id::<TypeFunctionGenericType>(self_ty);

    lua_check_tag!(
      vm_l,
      tfgt.is_null(),
      l,
      self_ty,
      "type.ispack: expected self to be a generic, but got {} instead"
    );

    lua_pushboolean(vm_l, (*tfgt).is_pack as i32);
    1
  }
}
