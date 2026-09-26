//! Source: `VM/src/lstate.cpp:28-48` (hand-ported)

use crate::{
  functions::c_slice_mut,
  macros::{
    basic_ci_size::BASIC_CI_SIZE, basic_stack_size::BASIC_STACK_SIZE, extra_stack::EXTRA_STACK,
    lua_m_newarray::luaM_newarray, lua_minstack::LUA_MINSTACK, setnilvalue::setnilvalue,
  },
  records::{call_info::CallInfo, lua_state::LuaState},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn stack_init(l1: *mut LuaState, l: *mut LuaState) {
  unsafe {
    // initialize CallInfo array
    (*l1).base_ci = luaM_newarray!(l, BASIC_CI_SIZE, CallInfo, (*l1).hdr.memcat);
    (*l1).ci = (*l1).base_ci;
    (*l1).size_ci = BASIC_CI_SIZE;
    (*l1).end_ci = (*l1).base_ci.add((*l1).size_ci as usize - 1);
    // initialize stack array
    (*l1).stack = luaM_newarray!(l, BASIC_STACK_SIZE + EXTRA_STACK, TValue, (*l1).hdr.memcat);
    (*l1).stacksize = BASIC_STACK_SIZE + EXTRA_STACK;
    let stack = (*l1).stack;
    // Safety:刚按 BASIC_STACK_SIZE + EXTRA_STACK 分配，全部可写。
    for slot in c_slice_mut(stack, (BASIC_STACK_SIZE + EXTRA_STACK) as usize) {
      setnilvalue!(slot); // erase new stack
    }
    (*l1).top = stack;
    (*l1).stack_last = stack.add(((*l1).stacksize - EXTRA_STACK) as usize);
    // initialize first ci
    (*(*l1).ci).func = (*l1).top;
    setnilvalue!((*l1).top); // `function' entry for this `ci'
    (*l1).top = (*l1).top.add(1);
    (*l1).base = (*l1).top;
    (*(*l1).ci).base = (*l1).top;
    (*(*l1).ci).top = (*l1).top.add(LUA_MINSTACK as usize);
  }
}
