use crate::{
  functions::{
    coclose::coclose_arm, cocreate::cocreate_arm, coresumecont::coresumecont_arm,
    coresumey::coresumey_arm, corunning::corunning_arm, costatus::costatus_arm, cowrap::cowrap_arm,
    coyield::coyield_arm, coyieldable::coyieldable_arm, lua_l_register::lua_l_register,
    lua_pushcclosurek::lua_pushcclosurek, lua_setfield::lua_setfield,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn luaopen_coroutine(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register(l, c"coroutine".as_ptr(), &CO_FUNCS);

    lua_pushcclosurek(
      l,
      Some(coresumey_arm),
      c"resume".as_ptr(),
      0,
      Some(coresumecont_arm),
    );
    lua_setfield(l, -2, c"resume".as_ptr());

    1
  }
}

lua_lib_fn!(pub(crate) fn luaopen_coroutine, luaopen_coroutine_arm);
static CO_FUNCS: [LuaLReg; 7] = [
  LuaLReg::new(b"create", cocreate_arm),
  LuaLReg::new(b"running", corunning_arm),
  LuaLReg::new(b"status", costatus_arm),
  LuaLReg::new(b"wrap", cowrap_arm),
  LuaLReg::new(b"yield", coyield_arm),
  LuaLReg::new(b"isyieldable", coyieldable_arm),
  LuaLReg::new(b"close", coclose_arm),
];
