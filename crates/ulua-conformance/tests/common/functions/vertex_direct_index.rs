use core::{
  ffi::{c_int, c_void},
  mem::size_of,
};

use ulua_vm::{
  functions::{
    lua_pushnumber::lua_pushnumber, lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32,
  },
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::{
  enums::direct_slot::DirectSlot,
  functions::{
    cstr_text::cstr_text, lua_vec_2_push::lua_vec_2_push, update_direct_slot::update_direct_slot,
  },
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vertex_direct_index(
  l: *mut LuaState,
  data: *mut c_void,
  atom: c_int,
  cachedslot: *mut u16,
  _utag: c_int,
) {
  // `data` 由 VM 交回本 userdata 的数据指针，直接改写成强类型局部（纯指针算术）。
  let self_ptr = data as *mut Vertex;

  // Safety: `cachedslot` 是 VM 为本直接访问访问点持有的可写 u16 槽位，本回调期间存活；
  // 槽位为 0 时按 cpp 由 `update_direct_slot` 补算（该门面自身为 safe）。
  unsafe {
    if *cachedslot == 0 {
      update_direct_slot(atom, cachedslot);
    }
  }

  // Safety: 同上——`cachedslot` 指向有效的 u16 槽位，此处只读一次用于分派。
  let slot = unsafe { *cachedslot };

  match DirectSlot::from_u16(slot) {
    Some(DirectSlot::Pos) => {
      // Safety: `self_ptr` 是 VM 交回的存活 Vertex 数据指针，pos 三分量可读。
      let pos = unsafe { (*self_ptr).pos };
      // Safety: `l` 存活；pushvector 只接收已取出的三个分量值。
      unsafe { lua_pushvector_lua_state_f32_f32_f32(l, pos[0], pos[1], pos[2]) }
    }
    Some(DirectSlot::Normal) => {
      // Safety: `self_ptr` 是 VM 交回的存活 Vertex 数据指针，normal 三分量可读。
      let normal = unsafe { (*self_ptr).normal };
      // Safety: `l` 存活；pushvector 只接收已取出的三个分量值。
      unsafe { lua_pushvector_lua_state_f32_f32_f32(l, normal[0], normal[1], normal[2]) }
    }
    Some(DirectSlot::UV) => {
      // Safety: `self_ptr` 存活，uv 两分量可读。
      let uv = unsafe { (*self_ptr).uv };
      // `lua_vec_2_push` 是 safe 门面：新建 Vec2 userdata 并返回其数据指针。
      let uv_data = lua_vec_2_push(l);
      // Safety: `uv_data` 为刚新建 userdata 的数据指针，可写两分量。
      unsafe {
        (*uv_data).x = uv[0];
        (*uv_data).y = uv[1];
      }
    }
    Some(DirectSlot::Sizeof) => {
      // Safety: `l` 存活；压入静态 `size_of` 结果，无指针解引用。
      unsafe { lua_pushnumber(l, size_of::<Vertex>() as f64) }
    }
    _ => {
      // Safety: `l` 存活；参数 2 为串时宏返回 NUL 结尾缓冲（否则抛 Lua 错误）。
      let name_ptr = unsafe { luaL_checkstring!(l, 2) };
      // Safety: 上一行保证 `name_ptr` 为 NUL 结尾串（lossy 渲染仅用于错误消息）。
      let name = unsafe { cstr_text(name_ptr) };
      // Safety: 按 cpp 抛「非成员」Lua 错误，该调用不返回。
      unsafe { luaL_error!(l, "{name} is not a valid member of vertex") }
    }
  }
}
