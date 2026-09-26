use ulua_vm::{
  functions::lua_namecallatom::lua_namecallatom,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::functions::{
  cstr_text::cstr_text, lua_vector_cross::lua_vector_cross, lua_vector_dot::lua_vector_dot,
};

/// # Safety
///
/// `l` 必须是指向有效 `LuaState` 的指针。
pub unsafe extern "C-unwind" fn lua_vector_namecall(l: *mut LuaState) -> i32 {
  let mut atom: i32 = 0;
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`atom` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  let str_ptr = unsafe { lua_namecallatom(l, &mut atom) };

  if !str_ptr.is_null() {
    // Safety: 上一步已排除 null，`lua_namecallatom` 保证其 NUL 结尾。
    let str_slice = unsafe { cstr_text(str_ptr) };

    if str_slice == "Dot" {
      return lua_vector_dot(l);
    }

    if str_slice == "Cross" {
      return lua_vector_cross(l);
    }
  }

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 为本用例存活的 LuaState，`luaL_checkstring!` 在参数 1 为串时返回 NUL 结尾缓冲。
  let arg1_ptr = unsafe { luaL_checkstring!(l, 1) };
  // Safety: 上一行保证 `arg1_ptr` 为 NUL 结尾串。
  let arg1_str = unsafe { cstr_text(arg1_ptr.cast()) };

  // `luaL_error!` 恒发散，作为尾表达式即可
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`atom` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { luaL_error!(l, "{} is not a valid method of vector", arg1_str) }
}
