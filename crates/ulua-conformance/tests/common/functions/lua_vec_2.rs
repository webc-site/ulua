use ulua_vm::{functions::lua_l_checknumber::lua_l_checknumber, records::lua_state::LuaState};

use crate::common::functions::lua_vec_2_push::lua_vec_2_push;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2(l: *mut LuaState) -> i32 {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let x = lua_l_checknumber(l, 1);
    let y = lua_l_checknumber(l, 2);

    let data = lua_vec_2_push(l);

    (*data).x = x as f32;
    (*data).y = y as f32;
  }
  1
}
