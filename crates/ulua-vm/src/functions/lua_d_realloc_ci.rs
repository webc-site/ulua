use core::mem::size_of;

use crate::{
  functions::lua_m_realloc::lua_m_realloc_,
  records::{call_info::CallInfo, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 LuaState 且 CallInfo 数组不变式成立：`base_ci <= ci <= end_ci`、`(*l).size_ci` 与已分配字节数一致；
/// `newsize` 须为正且不小于当前使用帧数（否则 `ci`/`end_ci` 重定位后越界）；`lua_m_realloc_` 可能返回新指针，
/// 旧 `base_ci` 之后即失效。cpp/VM/src/ldo.cpp:213 luaD_reallocCI。
pub unsafe fn lua_d_realloc_ci(l: *mut LuaState, newsize: i32) {
  unsafe {
    let oldci = (*l).base_ci;
    let oldoffset = (*l).ci.offset_from(oldci);

    (*l).base_ci = lua_m_realloc_(
      l,
      (*l).base_ci as *mut u8,
      (*l).size_ci as usize * size_of::<CallInfo>(),
      newsize as usize * size_of::<CallInfo>(),
      (*l).hdr.memcat,
    ) as *mut CallInfo;

    (*l).size_ci = newsize;
    (*l).ci = (*l).base_ci.offset(oldoffset);
    (*l).end_ci = (*l).base_ci.add(((*l).size_ci - 1) as usize);
  }
}
