use core::ffi::{c_int, c_void};

use ulua_vm::{
  functions::lua_l_checknumber::lua_l_checknumber,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::{
  enums::direct_slot::DirectSlot,
  functions::{cstr_text::cstr_text, update_direct_slot::update_direct_slot},
  records::vec_2_conformance_ir_hooks::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vec_2_direct_newindex(
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
      // Safety: `l` 存活；参数 3 为数字时返回值（否则抛 Lua 错误），`self_ptr` 存活可写 x。
      unsafe { (*self_ptr).x = lua_l_checknumber(l, 3) as f32 }
    }
    Some(DirectSlot::Y) => {
      // Safety: 同上——读参数 3 的数值，写 `self_ptr` 的 y 分量。
      unsafe { (*self_ptr).y = lua_l_checknumber(l, 3) as f32 }
    }
    _ => {
      // Safety: `l` 存活；参数 2 为串时宏返回 NUL 结尾缓冲（否则抛 Lua 错误）。
      let name_ptr = unsafe { luaL_checkstring!(l, 2) };
      // Safety: 上一行保证 `name_ptr` 为 NUL 结尾串（lossy 渲染仅用于错误消息）。
      let name = unsafe { cstr_text(name_ptr) };
      // Safety: 按 cpp 抛「不可写成员」Lua 错误，该调用不返回。
      unsafe { luaL_error!(l, "{name} is not a writable member of vec2") }
    }
  }
}
