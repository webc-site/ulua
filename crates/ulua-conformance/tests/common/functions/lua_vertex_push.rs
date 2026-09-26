use core::{ffi::c_int, mem::size_of};

use ulua_vm::{
  functions::{
    lua_getuserdatametatable::lua_getuserdatametatable,
    lua_newuserdatatagged::lua_newuserdatatagged, lua_setmetatable::lua_setmetatable,
  },
  records::lua_state::LuaState,
};

use crate::common::records::{userdata_tags::K_TAG_VERTEX, vertex::Vertex};

pub(crate) fn lua_vertex_push(l: *mut LuaState) -> *mut Vertex {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let data = lua_newuserdatatagged(l, size_of::<Vertex>(), K_TAG_VERTEX as c_int) as *mut Vertex;

    lua_getuserdatametatable(l, K_TAG_VERTEX as c_int);
    lua_setmetatable(l, -2);

    data
  }
}
