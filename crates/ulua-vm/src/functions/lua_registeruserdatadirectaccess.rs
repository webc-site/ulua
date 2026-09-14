use core::ffi::c_int;

use crate::{
  enums::tms::TMS,
  functions::lua_concat::lua_c_threadbarrier_lapi,
  macros::{api_check::api_check, fasttm::fasttm, lua_utag_limit::LUA_UTAG_LIMIT},
  records::lua_state::lua_State,
  type_aliases::{
    lua_userdata_direct_access::LuaUserdataDirectAccess,
    lua_userdata_direct_namecall::LuaUserdataDirectNamecall,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_registeruserdatadirectaccess(
  l: *mut lua_State,
  tag: c_int,
  get: LuaUserdataDirectAccess,
  set: LuaUserdataDirectAccess,
  namecall: LuaUserdataDirectNamecall,
) -> c_int {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    lua_c_threadbarrier_lapi(l);

    let h = (*(*l).global).udatamt[tag as usize];
    if !h.is_null() {
      let udatadirect = &mut (*(*l).global).udatadirect[tag as usize];

      let indextm = fasttm(l, h, TMS::TmIndex as c_int);
      if !indextm.is_null() {
        udatadirect.indextm = *indextm;
        udatadirect.index = get;
      }

      let newindextm = fasttm(l, h, TMS::TmNewIndex as c_int);
      if !newindextm.is_null() {
        udatadirect.newindextm = *newindextm;
        udatadirect.newindex = set;
      }

      let namecalltm = fasttm(l, h, TMS::TmNameCall as c_int);
      if !namecalltm.is_null() {
        udatadirect.namecalltm = *namecalltm;
        udatadirect.namecall = namecall;
      }

      return 1;
    }

    0
  }
}
