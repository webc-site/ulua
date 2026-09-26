use core::mem::size_of;

use ulua_vm::{
  functions::{lua_newuserdatatagged::lua_newuserdatatagged, lua_setmetatable::lua_setmetatable},
  macros::lua_l_getmetatable::lua_l_getmetatable,
  records::lua_state::LuaState,
};

use crate::common::functions::{cstr::cstr, k_int_64_tag::K_INT_64_TAG};
pub(crate) fn push_int_64(l: *mut LuaState, value: i64) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let p = lua_newuserdatatagged(l, size_of::<i64>(), K_INT_64_TAG);

    lua_l_getmetatable(l, cstr(b"int64\0"));
    lua_setmetatable(l, -2);

    *(p as *mut i64) = value;
  }
}
