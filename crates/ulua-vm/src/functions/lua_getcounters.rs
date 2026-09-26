use core::{ffi::c_void, ptr::addr_of};

use crate::{
  functions::{getcounters::getcounters, lua_a_toobject::lua_a_toobject},
  macros::api_check::api_check,
  records::{closure::LClosure, lua_state::LuaState},
  type_aliases::{
    lua_counter_function::LuaCounterFunction, lua_counter_value::LuaCounterValue, t_value::TValue,
  },
};

/// # Safety
/// `l` 必须指向存活 `LuaState`，且 `funcindex` 栈槽处值必须是 Lua（非 C）闭包 TValue——api_check 仅 debug
/// 兜底，release 下错型即按 LClosure 联合误读；`context` 与 `functionvisit`/`countervisit` 回调型别须匹配，
/// 回调将按原型指令序被反复调用。对应 cpp ldebug.cpp:647。
pub unsafe fn lua_getcounters(
  l: *mut LuaState,
  funcindex: i32,
  context: *mut c_void,
  functionvisit: LuaCounterFunction,
  countervisit: LuaCounterValue,
) {
  // Safety: 契约保证 `L` 存活、funcindex 处为 Lua 闭包可读，`counters` 为调用方可写输出数组且长度与原型指令数匹配
  unsafe {
    let func: *const TValue = lua_a_toobject(l, funcindex);
    api_check!(
      l,
      (*func).is_function() && (*(*func).as_closure_ptr()).is_c == 0
    );

    if (*(*l).global).ecb.getcounterdata.is_none() {
      return;
    }

    let cl = (*func).as_closure_ptr();
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    let p = (*lcl).p;

    getcounters(l, p, context, functionvisit, countervisit);
  }
}
