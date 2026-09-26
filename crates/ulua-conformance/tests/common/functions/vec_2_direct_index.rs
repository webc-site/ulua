use core::{
  ffi::{c_int, c_void},
  mem::size_of,
};

use ulua_vm::{
  functions::lua_pushnumber::lua_pushnumber,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::{
  enums::direct_slot::DirectSlot,
  functions::{
    cstr_text::cstr_text, lua_vec_2_push::lua_vec_2_push, update_direct_slot::update_direct_slot,
  },
  records::vec_2_conformance_ir_hooks::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vec_2_direct_index(
  l: *mut LuaState,
  data: *mut c_void,
  atom: c_int,
  cachedslot: *mut u16,
  _utag: c_int,
) {
  // `data` 由 VM 交回本 userdata 的数据指针，直接改写成强类型局部（纯指针算术）。
  let self_ptr = data as *mut Vec2;

  // Safety: `cachedslot` 是 VM 为本直接访问访问点持有的可写 u16 槽位，本回调期间存活；
  // 槽位为 0 时按 cpp 由 `update_direct_slot`（safe 门面）补算。
  unsafe {
    if *cachedslot == 0 {
      update_direct_slot(atom, cachedslot);
    }
  }

  // Safety: 同上——`cachedslot` 指向有效 u16 槽位，此处只读一次用于分派。
  let slot = unsafe { *cachedslot };

  match DirectSlot::from_u16(slot) {
    Some(DirectSlot::X) => {
      // Safety: `self_ptr` 是 VM 交回的存活 Vec2 数据指针，x 可读；`l` 存活。
      unsafe { lua_pushnumber(l, (*self_ptr).x as f64) }
    }
    Some(DirectSlot::Y) => {
      // Safety: `self_ptr` 是 VM 交回的存活 Vec2 数据指针，y 可读；`l` 存活。
      unsafe { lua_pushnumber(l, (*self_ptr).y as f64) }
    }
    Some(DirectSlot::Magnitude) => {
      // Safety: `self_ptr` 存活，两分量可读；压入的是纯算术结果。
      unsafe {
        lua_pushnumber(
          l,
          ((*self_ptr).x * (*self_ptr).x + (*self_ptr).y * (*self_ptr).y).sqrt() as f64,
        )
      }
    }
    Some(DirectSlot::Unit) => {
      // Safety: `self_ptr` 存活，两分量可读。
      let (x, y) = unsafe { ((*self_ptr).x, (*self_ptr).y) };
      let inv = 1.0 / (x * x + y * y).sqrt();
      // `lua_vec_2_push` 是 safe 门面：新建 Vec2 userdata 并返回其数据指针。
      let result = lua_vec_2_push(l);
      // Safety: `result` 为刚新建 userdata 的数据指针，可写两分量。
      unsafe {
        (*result).x = x * inv;
        (*result).y = y * inv;
      }
    }
    Some(DirectSlot::Sizeof) => {
      // Safety: `l` 存活；压入静态 `size_of` 结果，无指针解引用。
      unsafe { lua_pushnumber(l, size_of::<Vec2>() as f64) }
    }
    _ => {
      // Safety: `l` 存活；参数 2 为串时宏返回 NUL 结尾缓冲（否则抛 Lua 错误）。
      let name_ptr = unsafe { luaL_checkstring!(l, 2) };
      // Safety: 上一行保证 `name_ptr` 为 NUL 结尾串（lossy 渲染仅用于错误消息）。
      let name = unsafe { cstr_text(name_ptr) };
      // Safety: 按 cpp 抛「非成员」Lua 错误，该调用不返回。
      unsafe { luaL_error!(l, "{name} is not a valid member of vec2") }
    }
  }
}
