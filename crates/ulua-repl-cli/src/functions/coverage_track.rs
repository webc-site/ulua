use ulua_vm::{functions::lua_ref::lua_ref, type_aliases::lua_state::lua_State};

use crate::functions::coverage_init::G_COVERAGE;

// Faithful port of:
//     void coverage_track(lua_State* l, int funcindex) {
//         int ref = lua_ref(l, funcindex);
//         gCoverage.functions.push_back(ref);
//     }
pub fn coverage_track(l: *mut lua_State, funcindex: i32) {
  unsafe {
    let ref_id = lua_ref(l, funcindex);
    (*core::ptr::addr_of_mut!(G_COVERAGE))
      .functions
      .push(ref_id);
  }
}
