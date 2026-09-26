use core::ffi::{c_int, c_void};

use ulua_vm::{
  functions::lua_l_checkvector::lua_l_checkvector,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::{
  enums::direct_slot::DirectSlot,
  functions::{
    cstr_text::cstr_text, lua_vec_2_get::lua_vec_2_get, update_direct_slot::update_direct_slot,
  },
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vertex_direct_newindex(
  l: *mut LuaState,
  data: *mut c_void,
  atom: c_int,
  cachedslot: *mut u16,
  _utag: c_int,
) {
  // `data` 由 VM 交回本 userdata 的数据指针，直接改写成强类型局部（纯指针算术）。
  let self_ptr = data as *mut Vertex;

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
    Some(DirectSlot::Pos) => {
      // Safety: `l` 存活；参数 3 为 vector 时返回其 3 分量缓冲指针（否则抛 Lua 错误）。
      let pos = unsafe { lua_l_checkvector(l, 3) };
      // Safety: `self_ptr` 是 VM 交回的存活 Vertex 数据指针可写，`pos` 为 3 分量缓冲可读。
      unsafe {
        (*self_ptr).pos[0] = *pos.add(0);
        (*self_ptr).pos[1] = *pos.add(1);
        (*self_ptr).pos[2] = *pos.add(2);
      }
    }
    Some(DirectSlot::Normal) => {
      // Safety: `l` 存活；参数 3 为 vector 时返回其 3 分量缓冲指针（否则抛 Lua 错误）。
      let normal = unsafe { lua_l_checkvector(l, 3) };
      // Safety: `self_ptr` 存活可写，`normal` 为 3 分量缓冲可读。
      unsafe {
        (*self_ptr).normal[0] = *normal.add(0);
        (*self_ptr).normal[1] = *normal.add(1);
        (*self_ptr).normal[2] = *normal.add(2);
      }
    }
    Some(DirectSlot::UV) => {
      // Safety: `l` 存活；`lua_vec_2_get` 校验参数 3 为 Vec2 userdata 并交回数据指针。
      let uv = unsafe { lua_vec_2_get(l, 3) };
      // Safety: `uv` 指向存活 Vec2 数据（两分量可读），`self_ptr` 存活可写。
      unsafe {
        (*self_ptr).uv[0] = (*uv).x;
        (*self_ptr).uv[1] = (*uv).y;
      }
    }
    _ => {
      // Safety: `l` 存活；参数 2 为串时宏返回 NUL 结尾缓冲（否则抛 Lua 错误）。
      let name_ptr = unsafe { luaL_checkstring!(l, 2) };
      // Safety: 上一行保证 `name_ptr` 为 NUL 结尾串（lossy 渲染仅用于错误消息）。
      let name = unsafe { cstr_text(name_ptr) };
      // Safety: 按 cpp 抛「不可写成员」Lua 错误，该调用不返回。
      unsafe { luaL_error!(l, "{name} is not a writable member of vertex") }
    }
  }
}
