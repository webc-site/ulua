use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_pushboolean::lua_pushboolean, lua_pushnil::lua_pushnil},
  records::lua_state,
};

use crate::{
  enums::type_type_function_runtime::Type,
  functions::{
    get_tag::get_tag, get_type_function_runtime::get_type_function_type_id,
    get_type_user_data::get_type_user_data, push_string::push_string,
    throw_type_error::throw_type_error,
  },
  macros::{lua_check_args, lua_check_tag},
  records::{
    type_function_boolean_singleton::TypeFunctionBooleanSingleton,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_string_singleton::TypeFunctionStringSingleton,
  },
  type_aliases::lua_state::LuaState,
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int getSingletonValue(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:586`）。
pub unsafe fn get_singleton_value(l: *mut LuaState) -> i32 {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活，`l as *mut lua_state::LuaState` 为同址重解释。
  // `tfpt`/`tfst` 按 class-index 下转：`tfpt` 在 `!is_null()` 守卫后读 `(*tfpt).r#type`；`tfst`
  // 的 `is_null()` 分支内 `throw_type_error` 返回 `!` 不返回，故其后 `(*tfst).variant` 解引用合法，
  // `get_if::<..>()` 命中的子借用随该存活对象存在。末尾 `throw_type_error` 亦不返回。单线程串行，
  // 无并发别名。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    lua_check_args!(vm_l, != 1, "type.value: expected 1 argument, but got {}");

    let self_ty = get_type_user_data(l, 1);
    let tfpt = get_type_function_type_id::<TypeFunctionPrimitiveType>(self_ty);
    if !tfpt.is_null() {
      lua_check_tag!(
        vm_l,
        (*tfpt).r#type != Type::NilType,
        l,
        self_ty,
        "type.value: expected self to be a singleton, but got {} instead"
      );

      lua_pushnil(vm_l);
      return 1;
    }

    let tfst = get_type_function_type_id::<TypeFunctionSingletonType>(self_ty);
    lua_check_tag!(
      vm_l,
      tfst.is_null(),
      l,
      self_ty,
      "type.value: expected self to be a singleton, but got {} instead"
    );

    if let Some(tfbst) = (*tfst).variant.get_if::<TypeFunctionBooleanSingleton>() {
      lua_pushboolean(vm_l, tfbst.value as i32);
      return 1;
    }

    if let Some(tfsst) = (*tfst).variant.get_if::<TypeFunctionStringSingleton>() {
      push_string(l, &tfsst.value);
      return 1;
    }

    throw_type_error(
      vm_l,
      format_args!(
        "type.value: can't call `value` method on `{}` type",
        get_tag(l, self_ty)
      ),
    );
  }
}
