use ulua_vm::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_pushinteger::lua_pushinteger, lua_yield::lua_yield,
  },
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn c_yielding_iterator(l: *mut LuaState) -> i32 {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let max = lua_l_checkinteger(l, 1);
    let index = lua_l_checkinteger(l, 2);

    if index >= max {
      return 0; // nil: end iteration
    }

    lua_pushinteger(l, index + 1);
    lua_yield(l, 1)
  }
}
