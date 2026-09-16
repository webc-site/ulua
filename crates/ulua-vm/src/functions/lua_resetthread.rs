//! Node: `cxx:Function:Luau.VM:VM/src/lstate.cpp:148:lua_resetthread`
//! Source: `VM/src/lstate.cpp:148-180` (hand-ported)

use ulua_common::FFlag;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    c_slice_mut, cleanupcistack::cleanupcistack, lua_d_realloc_ci::lua_d_realloc_ci,
    lua_d_reallocstack::luaD_reallocstack, lua_f_close::lua_f_close,
  },
  macros::{
    api_check::api_check, basic_ci_size::BASIC_CI_SIZE, basic_stack_size::BASIC_STACK_SIZE,
    extra_stack::EXTRA_STACK, lua_minstack::LUA_MINSTACK, setnilvalue::setnilvalue,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_resetthread(l: *mut lua_State) {
  unsafe {
    api_check!(l, !(*l).isactive);
    api_check!(
      l,
      (*l).status != LuaStatus::Ok as u8 || (*l).ci == (*l).base_ci
    );

    // close upvalues before clearing anything
    lua_f_close(l, (*l).stack);
    if FFlag::LuauClosureUsageCounter.get() {
      cleanupcistack(l);
    }

    // clear call frames
    let ci = (*l).base_ci;
    (*ci).func = (*l).stack;
    (*ci).base = (*ci).func.add(1);
    (*ci).top = (*ci).base.add(LUA_MINSTACK as usize);
    setnilvalue!((*ci).func);
    (*l).ci = ci;
    if (*l).size_ci != BASIC_CI_SIZE {
      lua_d_realloc_ci(l, BASIC_CI_SIZE);
    }
    // clear thread state
    (*l).status = LuaStatus::Ok as u8;
    (*l).base = (*(*l).ci).base;
    (*l).top = (*(*l).ci).base;
    (*l).n_ccalls = 0;
    (*l).base_ccalls = 0;
    // clear thread stack
    if (*l).stacksize != BASIC_STACK_SIZE + EXTRA_STACK {
      luaD_reallocstack(l, BASIC_STACK_SIZE, 0);
    }
    // SAFETY：stack 数组长度为 stacksize（reallocstack 后仍保持一致）。
    for slot in c_slice_mut((*l).stack, (*l).stacksize as usize) {
      setnilvalue!(slot);
    }
  }
}
