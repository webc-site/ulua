use core::ffi::{c_int, c_void};

use crate::{
  functions::{lua_a_toobject::luaA_toobject, lua_m_freearray::getcounters},
  macros::{api_check::api_check, clvalue::clvalue, ttisfunction::ttisfunction},
  records::closure::LClosure,
  type_aliases::{
    lua_counter_function::LuaCounterFunction, lua_counter_value::LuaCounterValue,
    lua_state::lua_State, t_value::TValue,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_getcounters(
  l: *mut lua_State,
  funcindex: c_int,
  context: *mut c_void,
  functionvisit: LuaCounterFunction,
  countervisit: LuaCounterValue,
) {
  unsafe {
    let func: *const TValue = luaA_toobject(l, funcindex);
    api_check!(l, ttisfunction!(func) && (*clvalue!(func)).is_c == 0);

    if (*(*l).global).ecb.getcounterdata.is_none() {
      return;
    }

    let cl = clvalue!(func);
    let lcl = core::ptr::addr_of!((*cl).inner.l).cast::<LClosure>();
    let p = (*lcl).p;

    getcounters(l, p, context, functionvisit, countervisit);
  }
}
