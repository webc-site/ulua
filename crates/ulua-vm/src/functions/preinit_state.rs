use core::ptr::null_mut;

use crate::records::{global_state::global_State, lua_state::LuaState};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn preinit_state(l: *mut LuaState, g: *mut global_State) {
  unsafe {
    (*l).global = g;
    (*l).stack = null_mut();
    (*l).stacksize = 0;
    (*l).gt = null_mut();
    (*l).openupval = null_mut();
    (*l).size_ci = 0;
    (*l).n_ccalls = 0;
    (*l).base_ccalls = 0;
    (*l).status = 0;
    (*l).base_ci = null_mut();
    (*l).ci = null_mut();
    (*l).namecall = null_mut();
    (*l).cachedslot = 0;
    (*l).singlestep = false;
    (*l).isactive = false;
    (*l).activememcat = 0;
    (*l).userdata = null_mut();
  }
}
