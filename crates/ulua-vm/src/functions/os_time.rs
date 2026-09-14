use core::{ffi::c_int, ptr::null_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    getboolfield::getboolfield,
    getfield::getfield,
    localtime_r::{TimeT, Tm},
    lua_l_checktype::lua_l_checktype,
    lua_pushnil::lua_pushnil,
    lua_pushnumber::lua_pushnumber,
    lua_settop::lua_settop,
    os_timegm::os_timegm,
  },
  macros::lua_isnoneornil::lua_isnoneornil,
  type_aliases::lua_state::lua_State,
};

unsafe extern "C" {
  fn time(t: *mut TimeT) -> TimeT;
}

#[unsafe(export_name = "ulua_os_time")]
pub(crate) unsafe extern "C-unwind" fn os_time(l: *mut lua_State) -> c_int {
  unsafe {
    let t: i64 = if lua_isnoneornil!(l, 1) {
      time(null_mut())
    } else {
      let mut ts = Tm::default();

      lua_l_checktype(l, 1, LuaType::Table as c_int);
      lua_settop(l, 1);

      ts.tm_sec = getfield(l, "sec", 0);
      ts.tm_min = getfield(l, "min", 0);
      ts.tm_hour = getfield(l, "hour", 12);
      ts.tm_mday = getfield(l, "day", -1);
      // wrapping_sub avoids `int` underflow on an INT_MIN month/year (UB in
      // C++; panic with overflow-checks). os_timegm widens to i64 and the
      // `t == -1` path rejects out-of-range dates, so a wrapped field can't UB.
      ts.tm_mon = getfield(l, "month", -1).wrapping_sub(1);
      ts.tm_year = getfield(l, "year", -1).wrapping_sub(1900);
      ts.tm_isdst = getboolfield(l, "isdst");

      os_timegm(&ts)
    };

    if t == -1 {
      lua_pushnil(l);
    } else {
      lua_pushnumber(l, t as f64);
    }

    1
  }
}
