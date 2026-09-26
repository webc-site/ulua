use ulua_vm::{
  functions::{lua_l_checkstack::lua_l_checkstack, lua_pushnumber::lua_pushnumber},
  macros::luai_maxcstack::LUAI_MAXCSTACK,
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn slowly_overflow_stack(l: *mut LuaState) -> i32 {
  for _ in 0..(LUAI_MAXCSTACK * 2) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      lua_l_checkstack(l, 1, "test");
      lua_pushnumber(l, 1.0);
    }
  }
  0
}
