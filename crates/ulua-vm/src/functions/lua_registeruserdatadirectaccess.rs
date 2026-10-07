use crate::{
  enums::tms::TMS,
  functions::lapi_barrier::lua_c_threadbarrier_lapi,
  macros::{api_check::api_check, fasttm::fasttm, lua_utag_limit::LUA_UTAG_LIMIT},
  records::lua_state::LuaState,
  type_aliases::{
    lua_userdata_direct_access::LuaUserdataDirectAccess,
    lua_userdata_direct_namecall::LuaUserdataDirectNamecall,
  },
};

/// # Safety
/// `l` 指向存活 `LuaState`；`0 <= tag < LUA_UTAG_LIMIT`（cpp lapi.cpp:2057）；该 tag 的
/// 元表须先于本调用创建（`udatamt[tag]` 为空时仅返回 0 不写入）；`get/set/namecall` 须为
/// 在本状态存活期内有效的 C ABI 回调，写入 `udatadirect[tag]` 描述符后可被 VM 直取路径调用。
pub unsafe fn lua_registeruserdatadirectaccess(
  l: *mut LuaState,
  tag: i32,
  get: LuaUserdataDirectAccess,
  set: LuaUserdataDirectAccess,
  namecall: LuaUserdataDirectNamecall,
) -> i32 {
  // SAFETY: 契约保证 l 存活、tag 界内且回调为有效 C ABI 指针；块内只读元表 TM 并写 udatadirect[tag] 描述符
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    lua_c_threadbarrier_lapi(l);

    let h = (*l).gs_ref().udatamt[tag as usize];
    if !h.is_null() {
      // r16-b3 收编：原 `&mut (*(*l).global).udatadirect[..]` 绑定跨三次 `fasttm`
      // （经裸场域别名读 gs）持有，拆为逐写句 gs_mut 一句一借、原位现取；
      // 右值为 `fasttm` 返回裸指针的即时读数或入参，与 gs 无别名交叉
      let indextm = fasttm(l, h, TMS::TmIndex);
      if !indextm.is_null() {
        (*l).gs_mut().udatadirect[tag as usize].indextm = *indextm;
        (*l).gs_mut().udatadirect[tag as usize].index = get;
      }

      let newindextm = fasttm(l, h, TMS::TmNewIndex);
      if !newindextm.is_null() {
        (*l).gs_mut().udatadirect[tag as usize].newindextm = *newindextm;
        (*l).gs_mut().udatadirect[tag as usize].newindex = set;
      }

      let namecalltm = fasttm(l, h, TMS::TmNameCall);
      if !namecalltm.is_null() {
        (*l).gs_mut().udatadirect[tag as usize].namecalltm = *namecalltm;
        (*l).gs_mut().udatadirect[tag as usize].namecall = namecall;
      }

      return 1;
    }

    0
  }
}
