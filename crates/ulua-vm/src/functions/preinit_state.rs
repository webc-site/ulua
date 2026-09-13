use core::ptr::null_mut;

use crate::type_aliases::{global_state::global_State, lua_state::lua_State};

pub(crate) unsafe fn preinit_state(l: *mut lua_State, g: *mut global_State) {
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
