use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_l_typeerror_l::lua_l_typeerror_l, lua_touserdatatagged::lua_touserdatatagged},
  records::lua_state::LuaState,
};

use crate::common::records::{userdata_tags::K_TAG_VERTEX, vertex::Vertex};

pub(crate) fn lua_vertex_get(l: *mut LuaState, idx: i32) -> *mut Vertex {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let a = lua_touserdatatagged(l, idx, K_TAG_VERTEX as c_int) as *mut Vertex;

    if !a.is_null() {
      return a;
    }

    lua_l_typeerror_l(l, idx, "vertex");
  }
}
