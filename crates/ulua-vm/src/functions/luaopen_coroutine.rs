use crate::{
  functions::{
    coclose::coclose, cocreate::cocreate, coresumecont::coresumecont_arm, coresumey::coresumey,
    corunning::corunning, costatus::costatus, cowrap::cowrap, coyield::coyield,
    coyieldable::coyieldable, lua_l_register::lua_l_register, lua_pushcclosurek::lua_pushcclosurek,
    lua_setfield::lua_setfield,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn luaopen_coroutine(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register(l, c"coroutine".as_ptr(), &CO_FUNCS);

    lua_pushcclosurek(
      l,
      Some(coresumey),
      c"resume".as_ptr(),
      0,
      Some(coresumecont_arm),
    );
    lua_setfield(l, -2, c"resume".as_ptr());

    1
  }
}

static CO_FUNCS: [LuaLReg; 7] = [
  LuaLReg::new(b"create", cocreate),
  LuaLReg::new(b"running", corunning),
  LuaLReg::new(b"status", costatus),
  LuaLReg::new(b"wrap", cowrap),
  LuaLReg::new(b"yield", coyield),
  LuaLReg::new(b"isyieldable", coyieldable),
  LuaLReg::new(b"close", coclose),
];
