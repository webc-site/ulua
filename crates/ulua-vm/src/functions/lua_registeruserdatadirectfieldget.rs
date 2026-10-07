//! 直接字段取数回调的注册面（cpp `lua_registeruserdatadirectfieldget`，lapi.cpp:2193）。
//!
//! 裸指针保留：宿主运行期注册回调，类型集合非静态可穷举（review.md §4 保留条款）——
//! `fn_` 由宿主在 `LuauDirectFieldGet` 开启后按 (tag, field) 动态登记，落库为
//! `udatadirectfields[tag]` 哈希槽的 lightuserdata 载荷（解释器/ JIT 派发侧经
//! `from_ptr` 位模式还原），故其 `extern "C-unwind"` 函数指针 ABI 与 `l`/`field` 的
//! 指针形参一律维持原状：泛型化会封闭这张运行期开放注册表，把 `field` 改 `&str` 则
//! 把 NUL 结尾串的 C 契约推给宿主。可读入参已收口为一次切片扫描（`cstr_bytes`）。

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

/// 裸指针保留：宿主运行期注册回调，类型集合非静态可穷举（§4 保留条款，详见文件头）；
/// `fn_` 的 `extern "C-unwind"` ABI 与指针形参为协议固定面，不作泛型化或引用化改写。
///
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
  // SAFETY: 契约保证 l 存活、tag 界内、field 可读；建表/内化/写槽均在全局 udatadirectfields[tag] 界内
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
