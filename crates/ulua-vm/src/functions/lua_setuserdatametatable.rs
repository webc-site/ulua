//! `lua_setuserdatametatable` — pop a table from the stack and register it as
//! the metatable for userdata of type `tag`.
//! C++ source: `VM/src/lapi.cpp:1616`

use crate::{
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_utag_limit::LUA_UTAG_LIMIT,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setuserdatametatable(l: *mut LuaState, tag: i32) {
  unsafe {
    api_checknelems!(l, 1);
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    // reassignment not supported
    api_check!(l, (*l).gs_ref().udatamt[tag as usize].is_null());

    // r12-w7a2 收编：栈顶单槽预绑定读柄（既有形制确认——本函数体仅此一读）；
    // r12-w9b 续收：裸偏移读数改经 `top_slot(-1)` 边界原语，顶下界由上方
    // `api_checknelems!` 前置断言保证（契约见 top_slot），求值位点不动
    let t = (*l).top_slot(-1);
    let Some(h) = (*(*t).value.gc).as_table_mut() else {
      api_check!(l, false);
      return;
    };
    // r16-b3 收编：注册表槽写改经 gs_mut 一句一借；本句与上方 `top_slot`/下方
    // `rewind_top` 皆即取即用，无借用跨调用
    (*l).gs_mut().udatamt[tag as usize] = h as *mut LuaTable;

    // 弹栈经槽门面 rewind_top：t 即上方预绑定的当前栈顶单槽（其间仅标量/裸名
    // 注册表场写，无栈操作，现读场与窗值恒等），免二次场域重读
    (*l).rewind_top(1);
  }
}
