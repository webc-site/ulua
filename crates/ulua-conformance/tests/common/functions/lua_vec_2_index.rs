use core::mem::size_of;

use ulua_vm::{
  functions::lua_pushnumber::lua_pushnumber,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::{
  functions::{cstr_text::cstr_text, lua_vec_2_get::lua_vec_2_get, lua_vec_2_push::lua_vec_2_push},
  records::vec_2_conformance_ir_hooks::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_index(l: *mut LuaState) -> i32 {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 为本用例存活的 LuaState，`lua_vec_2_get`
  // 校验首参为 Vec2 userdata 并返回其数据指针。
  let v = unsafe { lua_vec_2_get(l, 1) };
  // Safety: `luaL_checkstring!` 在参数 2 为串时返回 NUL 结尾缓冲（否则抛 Lua 错误），
  // 指针至本行读取结束前有效。
  let name_ptr = unsafe { luaL_checkstring!(l, 2) };
  // Safety: 上一行的宏契约保证 `name_ptr` 为 NUL 结尾合法 C 串。
  let name = unsafe { cstr_text(name_ptr) };

  if name == "X" {
    // Safety: `v` 指向存活 Vec2 userdata 的数据；`lua_pushnumber` 只读该值。
    unsafe { lua_pushnumber(l, (*v).x as f64) };
    return 1;
  }

  if name == "Y" {
    // Safety: 同上——`v` 存活，读取 `y` 后压栈。
    unsafe { lua_pushnumber(l, (*v).y as f64) };
    return 1;
  }

  if name == "Magnitude" {
    // Safety: `v` 指向存活 Vec2 userdata 的数据，两分量可读。
    let (x, y) = unsafe { ((*v).x, (*v).y) };
    // Safety: `lua_pushnumber` 只接受已算好的值，`l` 存活。
    unsafe { lua_pushnumber(l, (x * x + y * y).sqrt() as f64) };
    return 1;
  }

  if name == "Unit" {
    // Safety: `v` 指向存活 Vec2 userdata 的数据，两分量可读。
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
    // Safety: `l` 存活；压入静态 `size_of` 结果，无指针解引用。
    unsafe { lua_pushnumber(l, size_of::<Vec2>() as f64) };
    return 1;
  }

  // Safety: 末分支按 cpp 抛 Lua 错误（宏内为 C ABI `luaL_error`，`l` 存活、格式串为
  // 已校验的 `name`）；该调用不返回。
  unsafe { luaL_error!(l, "{name} is not a valid member of vector") }
}
