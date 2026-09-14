use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_math_modf")]
pub(crate) unsafe extern "C-unwind" fn math_modf(l: *mut lua_State) -> i32 {
  unsafe {
    let mut ip: f64 = 0.0;
    let fp = (lua_l_checknumber(l, 1)).modf(&mut ip);
    lua_pushnumber(l, ip);
    lua_pushnumber(l, fp);
    2
  }
}

trait F64Modf {
  fn modf(&self, ip: &mut f64) -> f64;
}

impl F64Modf for f64 {
  fn modf(&self, ip: &mut f64) -> f64 {
    *ip = self.trunc();
    self - *ip
  }
}
