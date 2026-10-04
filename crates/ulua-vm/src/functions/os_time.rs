use coarsetime::Clock;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    getboolfield::getboolfield,
    getfield::getfield,
    localtime_r::{TimeT, Tm},
    os_timegm::os_timegm,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 挂钟 Unix 秒（替代 C `time(NULL)`）：`coarsetime` 的粗粒度缓存时钟，
/// 整秒截断与 C `time` 的取整语义一致；全目标纯 Rust（wasm32 经
/// `wasm-bindgen` 读宿主时钟，不再依赖旧的固定时刻 shim）。
pub(crate) fn now_epoch_seconds() -> TimeT {
  Clock::now_since_epoch().as_f64() as TimeT
}

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 取参/读表/压栈全经 `is_none_or_nil`/`check_type`/`set_top`/`getfield`/`getboolfield`/
/// `push_*` 安全门面与被调，体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错、可 GC 的
/// 受保护帧——索引 1 可空（`is_none_or_nil` 走 [`now_epoch_seconds`]），否则 `check_type` 要求
/// 为表否则抛错发散，`set_top(1)` 截顶后经 getfield/getboolfield 读表字段到本地 `Tm`
/// （不回写表）；末尾 `push_nil`/`push_number` 需 `top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/loslib.cpp:179
pub fn os_time(l: &mut LuaState) -> i32 {
  let t: i64 = if l.is_none_or_nil(1) {
    now_epoch_seconds()
  } else {
    let mut ts = Tm::default();

    l.check_type(1, LuaType::Table);
    l.set_top(1);

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
    l.push_nil();
  } else {
    l.push_number(t as f64);
  }

  1
}

lua_lib_fn!(pub fn os_time @ref, os_time_arm);
