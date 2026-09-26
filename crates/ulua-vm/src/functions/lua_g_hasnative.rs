use crate::{
  functions::getluaproto::get_lua_proto,
  records::{call_info::CallInfo, lua_state::LuaState, proto::Proto},
};

/// # Safety
///
/// `l` 须为有效存活的 `LuaState`，`level` 须指向当前调用栈内的活动层。
pub unsafe fn lua_g_hasnative(l: *mut LuaState, level: i32) -> i32 {
  // Safety: 契约保证 `g` 指向存活 global_State 且 cb.native 指针槽可读，无回调时返回空指针
  unsafe {
    if (level as u32) >= ((*l).ci).offset_from((*l).base_ci) as u32 {
      return 0;
    }

    let ci: *mut CallInfo = (*l).ci.offset(-(level as isize));
    let proto: *mut Proto = get_lua_proto(ci);
    if proto.is_null() {
      return 0;
    }

    ((*proto).execdata).is_null() as i32 ^ 1
  }
}
