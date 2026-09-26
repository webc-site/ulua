use core::{ffi::c_int, mem::size_of};

use ulua_vm::{
  functions::{
    lua_getuserdatametatable::lua_getuserdatametatable,
    lua_newuserdatatagged::lua_newuserdatatagged, lua_setmetatable::lua_setmetatable,
  },
  records::lua_state::LuaState,
};

use crate::common::records::{userdata_tags::K_TAG_VEC2, vec_2_conformance_ir_hooks::Vec2};

pub(crate) fn lua_vec_2_push(l: *mut LuaState) -> *mut Vec2 {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let data = lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_VEC2 as c_int) as *mut Vec2;

    lua_getuserdatametatable(l, K_TAG_VEC2 as c_int);

    lua_setmetatable(l, -2);

    data
  }
}
