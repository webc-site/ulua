use ulua_vm::{
  functions::{lua_pcall::lua_pcall, lua_pushnumber::lua_pushnumber},
  macros::{lua_getglobal::lua_getglobal, lua_isfunction::lua_isfunction},
  records::lua_state::LuaState,
};

use crate::common::{functions::cstr::cstr, records::vec_2_conformance_ir_hooks::Vec2};

pub(crate) fn lua_vec_2_reenter(l: *mut LuaState, self_ptr: *mut Vec2) -> i32 {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    lua_getglobal(l, cstr(b"reenterCallback\0"));
    assert!(lua_isfunction!(l, -1));
    lua_pcall(l, 0, 0, 0);

    let self_val = &*self_ptr;
    let result = (self_val.x as f64) + (self_val.y as f64);
    lua_pushnumber(l, result);
  }
  1
}
