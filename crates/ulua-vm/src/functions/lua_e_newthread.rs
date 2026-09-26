//! Source: `VM/src/lstate.cpp:116-128` (hand-ported)

use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::lua_m_newgco, preinit_state::preinit_state, stack_init::stack_init},
  macros::{iswhite::iswhite, lua_c_init::luaC_init},
  records::{gc_object::GcObject, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活的父 `LuaState`（`(*l).global`、`activememcat`、`gt` 有效），`lua_m_newgco` 分配新线程
/// 并可触发 GC；新线程 `l1` 经 `preinit_state`/`stack_init` 初始化后方返回，`stack_init` 会写 `l1` 各栈字段。
/// 调用方须接住返回的白色对象指针（防回收）。cpp `lstate.cpp:116`。
pub unsafe fn lua_e_newthread(l: *mut LuaState) -> *mut LuaState {
  unsafe {
    let parent = &*l;
    let l1 = lua_m_newgco(l, size_of::<LuaState>(), parent.activememcat) as *mut LuaState;

    luaC_init!(l, l1, LuaType::Thread as i32);
    preinit_state(l1, parent.global);
    (*l1).activememcat = parent.activememcat; // inherit the active memory category
    stack_init(l1, l); // init stack
    (*l1).gt = parent.gt; // share table of globals
    (*l1).singlestep = parent.singlestep;
    LUAU_ASSERT!(iswhite!(l1 as *mut GcObject)); // iswhite(obj2gco(l1))
    l1
  }
}
