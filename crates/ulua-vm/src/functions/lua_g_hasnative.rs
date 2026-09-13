use core::ffi::c_int;

use crate::{
  functions::getluaproto::get_lua_proto,
  records::{call_info::CallInfo, lua_state::lua_State},
  type_aliases::proto::Proto,
};

#[unsafe(export_name = "ulua_lua_g_hasnative")]
pub(crate) unsafe fn lua_g_hasnative(l: *mut lua_State, level: c_int) -> c_int {
  unsafe {
    if (level as u32) >= ((*l).ci).offset_from((*l).base_ci) as u32 {
      return 0;
    }

    let ci: *mut CallInfo = (*l).ci.offset(-(level as isize));
    let proto: *mut Proto = get_lua_proto(ci);
    if proto.is_null() {
      return 0;
    }

    ((*proto).execdata).is_null() as c_int ^ 1
  }
}
