use coarsetime::Clock;

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
  macros::{lua_isnoneornil::lua_isnoneornil, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// 挂钟 Unix 秒（替代 C `time(NULL)`）：`coarsetime` 的粗粒度缓存时钟，
/// 整秒截断与 C `time` 的取整语义一致；全目标纯 Rust（wasm32 经
/// `wasm-bindgen` 读宿主时钟，不再依赖旧的固定时刻 shim）。
pub(crate) fn now_epoch_seconds() -> TimeT {
  Clock::now_since_epoch().as_f64() as TimeT
}

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：索引 1 可空（`lua_isnoneornil` 走 [`now_epoch_seconds`]），否则
/// `luaL_checktype(l,1,TABLE)` 要求为表否则抛错回退，`lua_settop(l,1)` 截顶后经 getfield/getboolfield 读表字段
/// 到本地 `Tm`（不回写表）；末尾 `lua_pushnil`/`lua_pushnumber` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/loslib.cpp:179
pub unsafe fn os_time(l: *mut LuaState) -> i32 {
  unsafe {
    let t: i64 = if lua_isnoneornil!(l, 1) {
      now_epoch_seconds()
    } else {
      let mut ts = Tm::default();

      lua_l_checktype(l, 1, LuaType::Table as i32);
      lua_settop(l, 1);

      ts.tm_sec = getfield(l, b"sec", 0);
      ts.tm_min = getfield(l, b"min", 0);
      ts.tm_hour = getfield(l, b"hour", 12);
      ts.tm_mday = getfield(l, b"day", -1);
      // wrapping_sub avoids `int` underflow on an INT_MIN month/year (UB in
      // C++; panic with overflow-checks). os_timegm widens to i64 and the
      // `t == -1` path rejects out-of-range dates, so a wrapped field can't UB.
      ts.tm_mon = getfield(l, b"month", -1).wrapping_sub(1);
      ts.tm_year = getfield(l, b"year", -1).wrapping_sub(1900);
      ts.tm_isdst = getboolfield(l, b"isdst");

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

lua_lib_fn!(pub fn os_time, os_time_arm);
