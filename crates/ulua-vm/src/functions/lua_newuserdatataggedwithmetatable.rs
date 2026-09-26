use core::ffi::c_void;

use crate::{functions::lua_newuserdatatagged::new_udata_impl, records::lua_state::LuaState};

/// # Safety
/// `l` 必须指向存活 `LuaState`；`tag` 须满足 0 <= tag < LUA_UTAG_LIMIT（api_check 仅 debug 兜底，越界即
/// udatamt[] 越界读并把错 metatable 挂上对象）；返回的裸 userdata 指针受 GC 管辖，调用方不得在其所属对象被
/// 回收后继续使用。对应 cpp lapi.cpp:1650。
pub unsafe fn lua_newuserdatataggedwithmetatable(
  l: *mut LuaState,
  sz: usize,
  tag: i32,
) -> *mut c_void {
  // Safety: 转发共享核心；`with_metatable = true` 挂 `udatamt[tag]` 元表并收紧行 proxy 放行
  unsafe { new_udata_impl(l, sz, tag, true) }
}
