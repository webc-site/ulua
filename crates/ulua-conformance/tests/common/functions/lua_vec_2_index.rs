use core::mem::size_of;

use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::{
  functions::{lua_vec_2_get::lua_vec_2_get, lua_vec_2_push::lua_vec_2_push, safe_api::state_mut},
  records::vec_2_conformance_ir_hooks::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_index(l: *mut LuaState) -> i32 {
  // `lua_vec_2_get` 是 safe 门面：校验首参为 Vec2 userdata 并返回其数据指针。
  let v = lua_vec_2_get(l, 1);
  let name = state_mut(l).check_str(2);

  if name == "X" {
    // Safety: `v` 指向存活 Vec2 userdata 的数据，仅本行读一次。
    let x = unsafe { (*v).x };
    state_mut(l).push_number(x as f64);
    return 1;
  }

  if name == "Y" {
    // Safety: 同上——读 `y` 一次。
    let y = unsafe { (*v).y };
    state_mut(l).push_number(y as f64);
    return 1;
  }

  if name == "Magnitude" {
    // Safety: 同上——两分量各读一次。
    let (x, y) = unsafe { ((*v).x, (*v).y) };
    state_mut(l).push_number((x * x + y * y).sqrt() as f64);
    return 1;
  }

  if name == "Unit" {
    // Safety: 同上——两分量各读一次。
    let (x, y) = unsafe { ((*v).x, (*v).y) };
    let inv_sqrt = 1.0 / (x * x + y * y).sqrt();

    // `lua_vec_2_push` 是 safe 门面：在 `l` 上新建 Vec2 userdata 并返回其数据指针。
    let data = lua_vec_2_push(l);
    // Safety: `data` 为刚新建 userdata 的数据指针，可写两分量。
    unsafe {
      (*data).x = x * inv_sqrt;
      (*data).y = y * inv_sqrt;
    };
    return 1;
  }

  if name == "sizeof" {
    // 压入静态 `size_of` 结果，无指针解引用。
    state_mut(l).push_number(size_of::<Vec2>() as f64);
    return 1;
  }

  // 末分支按 cpp 抛 Lua 错误（宏内为 C ABI `luaL_error`，格式串为已校验的
  // `name`）；该调用不返回。
  // Safety: `l` 存活；`luaL_error` 以 long-jump 终止本回调。
  unsafe { luaL_error!(l, "{name} is not a valid member of vector") }
}
