use core::{mem::size_of, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::lua_m_newgco, lua_m_toobig::lua_m_toobig},
  macros::{lua_c_init::luaC_init, sizeudata::sizeudata},
  records::{lua_state::LuaState, udata::Udata},
};

/// # Safety
/// `l` 须为存活 `LuaState`：`s` 为业务字节数，须满足 `s ≤ i32::MAX-size_of::<Udata>()`（超限走
/// `lua_m_toobig` 抛错，需处于受保护帧）；`lua_m_newgco` 以 `(*l).activememcat` 分配并可能触发 GC，返回指针已
/// 经 `luaC_init!` 接住防回收；`tag` 须落在 `0..=255`（LUAU_ASSERT 校验）后写入 `Udata.tag`。
/// cpp VM/src/ludata.cpp:10
pub unsafe fn lua_u_newudata(l: *mut LuaState, s: usize, tag: i32) -> *mut Udata {
  unsafe {
    if s > i32::MAX as usize - size_of::<Udata>() {
      lua_m_toobig(l);
    }

    let u = lua_m_newgco(l, sizeudata(s), (*l).activememcat) as *mut Udata;
    luaC_init!(l, u, LuaType::UserData as i32);
    let ud = &mut *u;
    ud.len = s as i32;
    ud.metatable = null_mut();
    LUAU_ASSERT!((0..=255).contains(&tag));
    ud.tag = tag as u8;
    u
  }
}

/// # Safety
/// 导出壳，契约同 `lua_u_newudata`：`l` 存活且处于受保护帧、`s ≤ i32::MAX-size_of::<Udata>()`、`tag∈0..=255`。
pub unsafe extern "C-unwind" fn lua_u_newudata_export(
  l: *mut LuaState,
  s: usize,
  tag: i32,
) -> *mut Udata {
  // Safety: 导出壳原样转发同契约 `lua_u_newudata`；`l` 存活、s 为分配字节数、tag 在注册界内
  unsafe { lua_u_newudata(l, s, tag) }
}
