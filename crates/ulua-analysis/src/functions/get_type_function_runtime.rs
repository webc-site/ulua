use ulua_vm::{
  functions::{lua_getthreaddata::lua_getthreaddata, lua_mainthread::lua_mainthread},
  type_aliases::lua_state,
};

use crate::{
  records::type_function_runtime::TypeFunctionRuntime, type_aliases::lua_state::LuaState,
};
/// # Safety
/// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn get_type_function_runtime(l: *mut LuaState) -> *mut TypeFunctionRuntime {
  unsafe {
    let main_thread = lua_mainthread(l as *mut lua_state::LuaState);
    let data = lua_getthreaddata(main_thread);
    data as *mut TypeFunctionRuntime
  }
}
