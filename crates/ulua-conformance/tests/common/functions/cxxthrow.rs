use core::ffi::c_int;

use ulua_vm::{
  macros::{lua_l_error::luaL_error, lua_use_longjmp::LUA_USE_LONGJMP},
  records::lua_state::LuaState,
};

/// C-unwind ABI：LUA_USE_LONGJMP=0 时跨 C 栈帧 panic 展开是定义行为。
pub(crate) extern "C-unwind" fn cxxthrow(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    if LUA_USE_LONGJMP != 0 {
      luaL_error!(l, "oops");
    } else {
      panic!("oops");
    }
  }
}
