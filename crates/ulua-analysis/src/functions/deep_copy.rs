

/// 对应 C++ 原生 `static int deepCopy(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:1865`）。
use ulua_common::fflag;
use crate::{functions::{alloc_type_user_data::alloc_type_user_data, deep_clone::deep_clone, get_type_function_runtime::get_type_function_runtime, get_type_user_data::get_type_user_data, throw_type_error::throw_type_error}, macros::lua_check_args, records::arena_handle::Handle};
use ulua_vm::records::lua_state::LuaState;
pub(crate) fn deep_copy(l: &mut LuaState) -> i32 {
  unsafe {
    lua_check_args!(l, != 1, "types.copy: expected 1 arguments, but got {}");

    let arg = get_type_user_data(&mut *l, 1);
    // runtime 由注册阶段挂在本线程 userdatum 上，deepCopy 只在已接线的类型函数环境里被
    // 调用，故恒非空；`Handle::from_ptr` 把该契约从「解引用即 UB」收为「违例即确定性 panic」。
    let runtime = Handle::from_ptr(get_type_function_runtime(&mut *l));
    let copy = deep_clone(runtime, arg);

    if fflag::LuauTypeFunctionRobustness.get() && copy.is_null() {
      throw_type_error(
        l.as_mut_ptr(),
        format_args!("types.copy: complexity limit reached during type copy"),
      );
    }

    alloc_type_user_data(&mut *l, (*copy).type_variant.clone(), false);
    1
  }
}
