//! Node: `cxx:Function:Luau.VM:VM/src/lstate.cpp:28:stack_init`
//! Source: `VM/src/lstate.cpp:28-48` (hand-ported)

use crate::{
  functions::c_slice_mut,
  macros::{
    basic_ci_size::BASIC_CI_SIZE, basic_stack_size::BASIC_STACK_SIZE, extra_stack::EXTRA_STACK,
    lua_m_newarray::luaM_newarray, lua_minstack::LUA_MINSTACK, setnilvalue::setnilvalue,
  },
  records::call_info::CallInfo,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

pub(crate) unsafe fn stack_init(l1: *mut lua_State, l: *mut lua_State) {
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
    // SAFETY：刚按 BASIC_STACK_SIZE + EXTRA_STACK 分配，全部可写。
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
