//! Node: `cxx:Function:Luau.VM:VM/src/lstate.cpp:116:luaE_newthread`
//! Source: `VM/src/lstate.cpp:116-128` (hand-ported)

use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::luaM_newgco_, preinit_state::preinit_state, stack_init::stack_init},
  macros::{iswhite::iswhite, lua_c_init::luaC_init},
  records::gc_object::GcObject,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_e_newthread(l: *mut lua_State) -> *mut lua_State {
  unsafe {
    let l1 = luaM_newgco_(l, size_of::<lua_State>(), (*l).activememcat) as *mut lua_State;
    luaC_init!(l, l1, LuaType::Thread as i32);
    preinit_state(l1, (*l).global);
    (*l1).activememcat = (*l).activememcat; // inherit the active memory category
    stack_init(l1, l); // init stack
    (*l1).gt = (*l).gt; // share table of globals
    (*l1).singlestep = (*l).singlestep;
    LUAU_ASSERT!(iswhite!(l1 as *mut GcObject)); // iswhite(obj2gco(l1))
    l1
  }
}

pub use lua_e_newthread as luaE_newthread;
