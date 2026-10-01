use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::{lua_vec_2_get::lua_vec_2_get, lua_vec_2_push::lua_vec_2_push},
  records::vec_2_conformance_ir_hooks::Vec2,
};

pub(crate) fn lua_vec_2_min(l: *mut LuaState, self_ptr: *mut Vec2) -> i32 {
  // Safety: `l` 存活；`lua_vec_2_get` 校验参数 2 为 Vec2 userdata 并返回其数据指针。
  let b_ptr = unsafe { lua_vec_2_get(l, 2) };
  // `lua_vec_2_push` 是 safe 门面：新建 Vec2 userdata 并返回其数据指针。
  let data = lua_vec_2_push(l);

  // Safety: 两个指针分别指向刚校验的 self/参数 userdata，读取期间均存活。
  let (sx, sy) = unsafe { ((*self_ptr).x, (*self_ptr).y) };
  // Safety: 同上——`b_ptr` 指向存活 Vec2 数据。
  let (bx, by) = unsafe { ((*b_ptr).x, (*b_ptr).y) };

  // 逐分量取小：与 cpp 一致用 `<` 比较（NaN 时取参数分量），不用 `min` 门面。
  let x = if sx < bx { sx } else { bx };
  let y = if sy < by { sy } else { by };

  // Safety: `data` 为刚新建 userdata 的数据指针，可写两分量。
  unsafe {
    (*data).x = x;
    (*data).y = y;
  }
  1
}
