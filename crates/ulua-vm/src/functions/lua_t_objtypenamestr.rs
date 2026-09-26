use core::ffi::c_void;

use crate::{
  enums::tms::TMS,
  functions::lua_h_getstr::lua_h_getstr,
  macros::{
    lightuserdatatag::lightuserdatatag, lua_lutag_limit::LUA_LUTAG_LIMIT, ttype::ttype,
    utag_proxy::UTAG_PROXY,
  },
  records::{lua_state::LuaState, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 调用方须保证：`l` 存活且 global 的 ttname/mt/tmname/lightuserdataname 数组已按类型标签初始化、
/// `o` 指向可读 TValue（ttype 标签 < LUA_T_COUNT 且 userdata 的 tag 可信）；返回存活 TString，
/// 其有效性止于后续 GC 或 lightuserdataname 被改写。cpp ltm.cpp:140 `luaT_objtypenamestr`
pub(crate) unsafe fn lua_t_objtypenamestr(l: *mut LuaState, o: *const TValue) -> *const tstring {
  unsafe {
    // Userdata created by the environment can have a custom type name set in the individual metatable
    // If there is no custom name, 'userdata' is returned
    if (*o).is_userdata() {
      let u = (*o).as_userdata_ptr();
      let mt = (*u).metatable;
      if (*u).tag as i32 != UTAG_PROXY && !mt.is_null() {
        let type_ = lua_h_getstr(mt, (*(*l).global).tmname[TMS::TmType as usize]);

        if (*type_).is_string() {
          return (*type_).as_string_ptr();
        }

        return (*(*l).global).ttname[ttype!(o) as usize];
      }
    }

    // Tagged lightuserdata can be named using lua_setlightuserdataname
    if (*o).is_lightuserdata() {
      let tag = lightuserdatatag!(o);

      if (tag as u32) < LUA_LUTAG_LIMIT as u32 {
        let name = (*(*l).global).lightuserdataname[tag as usize];
        if !name.is_null() {
          return name;
        }
      }
    }

    // For all types except userdata and table, a global metatable can be set with a global name override
    let mt = (*(*l).global).mt[ttype!(o) as usize];
    if !mt.is_null() {
      let type_ = lua_h_getstr(mt, (*(*l).global).tmname[TMS::TmType as usize]);

      if (*type_).is_string() {
        return (*type_).as_string_ptr();
      }
    }

    (*(*l).global).ttname[ttype!(o) as usize]
  }
}

/// # Safety
/// 与 [`lua_t_objtypenamestr`] 同契约：`l` 存活、global 名称数组完整、`o` 为可读 TValue；
/// 返回的 TString 指针以 `c_void` 宽化，有效性随 GC 结束。cpp ltm.cpp:140
pub unsafe extern "C-unwind" fn lua_t_objtypenamestr_export(
  l: *mut LuaState,
  o: *const TValue,
) -> *const c_void {
  // Safety: 导出壳原样转发同契约 `lua_t_objtypenamestr`，返回 C 字符串指针按 c_void 宽化
  unsafe { lua_t_objtypenamestr(l, o).cast() }
}
