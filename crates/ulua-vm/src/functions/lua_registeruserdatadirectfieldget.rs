use core::ffi::{c_char, c_void};

use ulua_common::fflag;

use crate::{
  functions::{cstr_bytes, lua_h_new::lua_h_new, lua_h_setstr::lua_h_setstr},
  macros::{
    api_check::api_check, fixedbit::FIXEDBIT, l_setbit::l_setbit, lua_s_new::lua_s_new,
    lua_utag_limit::LUA_UTAG_LIMIT, setpvalue::setpvalue,
  },
  records::{global_state::global_State, lua_state::LuaState, t_string::tstring},
  type_aliases::{lua_userdata_direct_field_get::LuaUserdataDirectFieldGet, t_value::TValue},
};

/// # Safety
/// `l` 指向存活 `LuaState`；`0 <= tag < LUA_UTAG_LIMIT`；`field` 指向本次调用内有效的
/// NUL 结尾串（立即经 `lua_s_new` 内化）；`fn_` 须为与 `LuaUserdataDirectFieldGet` 签名兼容、
/// 在注册表项存续期内有效的 C ABI 回调。cpp lapi.cpp:2193.
pub unsafe fn lua_registeruserdatadirectfieldget(
  l: *mut LuaState,
  tag: i32,
  field: *const c_char,
  fn_: LuaUserdataDirectFieldGet,
) {
  // Safety: 契约保证 l 存活、tag 界内、field 可读；建表/内化/写槽均在全局 udatadirectfields[tag] 界内
  unsafe {
    if !fflag::LuauDirectFieldGet.get() {
      return;
    }

    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    api_check!(l, !field.is_null());
    api_check!(l, fn_.is_some());

    let g: *mut global_State = (*l).global;

    if (*g).udatadirectfields[tag as usize].is_null() {
      (*g).udatadirectfields[tag as usize] = lua_h_new(l, 0, 1);
    }

    // 入参为 NUL 结尾 C 串，经 cstr_bytes 扫首个 NUL 得字节切片（保持原 lua_s_new 的 strlen 语义）
    let ts: *mut tstring = lua_s_new(l, cstr_bytes(field));
    l_setbit!((*ts).hdr.marked, FIXEDBIT);

    let slot: *mut TValue = lua_h_setstr(l, (*g).udatadirectfields[tag as usize], ts);
    // fn_ 非空由 API 契约保证（api_check! 对应 cpp: api_check(L, fn != nullptr)），
    // 且安全 Rust 无空 fn 指针可构造，expect 即最贴合 cpp 直调语义的落点
    let code =
      fn_.expect("注册期 API 契约保证直接字段 getter fn_ 非空（cpp: api_check(L, fn != nullptr)）");
    setpvalue!(slot, code as *mut c_void, 0);
  }
}
