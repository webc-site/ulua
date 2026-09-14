use core::{ffi::c_int, ops::Deref, ptr::null};

use crate::{
  functions::{
    coclose::coclose, cocreate::cocreate, coresumecont::coresumecont, coresumey::coresumey,
    corunning::corunning, costatus::costatus, cowrap::cowrap, coyield::coyield,
    coyieldable::coyieldable, lua_l_register::lua_l_register, lua_pushcclosurek::lua_pushcclosurek,
    lua_setfield::lua_setfield,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn luaopen_coroutine(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_register(l, c"coroutine".as_ptr(), CO_FUNCS.0.as_ptr());

    lua_pushcclosurek(
      l,
      Some(coresumey),
      c"resume".as_ptr(),
      0,
      Some(coresumecont),
    );
    lua_setfield(l, -2, c"resume".as_ptr());

    1
  }
}

struct SyncLuaLReg([LuaLReg; 8]);
unsafe impl Sync for SyncLuaLReg {}

static CO_FUNCS: SyncLuaLReg = SyncLuaLReg([
  LuaLReg {
    name: c"create".as_ptr(),
    func: Some(cocreate),
  },
  LuaLReg {
    name: c"running".as_ptr(),
    func: Some(corunning),
  },
  LuaLReg {
    name: c"status".as_ptr(),
    func: Some(costatus),
  },
  LuaLReg {
    name: c"wrap".as_ptr(),
    func: Some(cowrap),
  },
  LuaLReg {
    name: c"yield".as_ptr(),
    func: Some(coyield),
  },
  LuaLReg {
    name: c"isyieldable".as_ptr(),
    func: Some(coyieldable),
  },
  LuaLReg {
    name: c"close".as_ptr(),
    func: Some(coclose),
  },
  LuaLReg {
    name: null(),
    func: None,
  },
]);

impl Deref for SyncLuaLReg {
  type Target = [LuaLReg; 8];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
