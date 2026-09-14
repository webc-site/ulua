use core::ffi::c_void;

use crate::{
  functions::lua_v_getimport::lua_v_getimport,
  macros::{lua_d_checkstack::luaD_checkstack, setnilvalue::setnilvalue},
  records::resolve_import::ResolveImport,
  type_aliases::lua_state::lua_State,
};

impl ResolveImport {
  pub(crate) unsafe extern "C-unwind" fn run(l: *mut lua_State, ud: *mut c_void) {
    unsafe {
      let self_ = ud as *mut ResolveImport;

      // note: we call getimport with nil propagation which means that accesses to table chains like A.B.C will resolve in nil
      // this is technically not necessary but it reduces the number of exceptions when loading scripts that rely on getfenv/setfenv for global
      // injection
      // allocate a stack slot so that we can do table lookups
      luaD_checkstack!(l, 1);
      setnilvalue!((*l).top);
      (*l).top = (*l).top.add(1);

      lua_v_getimport(
        l,
        (*l).gt,
        (*self_).k,
        (*l).top.sub(1),
        (*self_).id,
        true, /* propagatenil= */
      );
    }
  }
}
