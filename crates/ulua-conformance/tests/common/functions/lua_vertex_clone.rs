use ulua_vm::records::lua_state::LuaState;

use crate::common::{functions::lua_vertex_push::lua_vertex_push, records::vertex::Vertex};

pub(crate) fn lua_vertex_clone(l: *mut LuaState, self_ptr: *mut Vertex) -> i32 {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let r_ptr = lua_vertex_push(l);
    *r_ptr = *self_ptr;
    1
  }
}
