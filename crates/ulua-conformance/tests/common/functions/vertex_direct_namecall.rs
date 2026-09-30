use core::{
  ffi::{c_int, c_void},
  ptr::null_mut,
};

use ulua_vm::{
  functions::lua_namecallatom::lua_namecallatom, macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

use crate::common::{
  enums::direct_slot::DirectSlot,
  functions::{
    cstr_text::cstr_text, lua_vertex_clone::lua_vertex_clone,
    update_direct_slot::update_direct_slot,
  },
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vertex_direct_namecall(
  l: *mut LuaState,
  data: *mut c_void,
  atom: c_int,
  cachedslot: *mut u16,
  _utag: c_int,
) -> c_int {
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
    // Clone 是 safe 门面（自行压栈并给出返回计数）。
    Some(DirectSlot::Clone) => lua_vertex_clone(l, self_ptr),
    _ => {
      // Safety: `l` 存活；`lua_namecallatom` 返回 namecall 方法名的 NUL 结尾指针或 null
      // （第二实参 null_mut() 表示不取 arg 计数）。
      // FFI: c-API 要求 NULL
      let method = unsafe { lua_namecallatom(l, null_mut()) };
      // Safety: 空指针按空串处理；非空时 `lua_namecallatom` 保证 NUL 结尾。
      let method = unsafe { cstr_text(method) };
      // Safety: 按 cpp 抛「非方法」Lua 错误（`l` 存活、格式串为已校验的 `method`），
      // 该调用不返回。
      unsafe { luaL_error!(l, "{method} is not a valid method of vertex") }
    }
  }
}
