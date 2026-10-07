use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_f_closeupval::lua_f_closeupval,
  macros::{isblack::isblack, isdead::isdead, upisopen::upisopen},
  records::{gc_object::GCObject, lua_state::LuaState, up_val::UpVal},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `openupval` 链自洽（链上各 `UpVal` 存活、`v` 为可比较裸栈
/// 指针）；`level` 为 `l` 栈内合法槽位界点。逐槽关闭经 `lua_f_closeupval`，其前提由链
/// 不变量承载。cpp lfunc.cpp:140。
pub(crate) unsafe fn lua_f_close(l: *mut LuaState, level: StkId) {
  unsafe {
    let g = (*l).global;

    while !(*l).openupval.is_null() && (*(*l).openupval).v >= level {
      let uv: *mut UpVal = (*l).openupval;
      let o = uv as *mut GCObject;
      LUAU_ASSERT!(!isblack!(o) && upisopen!(uv));
      LUAU_ASSERT!(!isdead!(g, o));

      (*l).openupval = (*uv).u.open.threadnext;
      lua_f_closeupval(l, uv, false);
    }
  }
}

/// # Safety
/// C ABI 导出壳：`l`/`level` 原样透传，须满足 [`lua_f_close`] 的全部前提。
pub unsafe extern "C-unwind" fn lua_f_close_export(l: *mut LuaState, level: StkId) {
  unsafe {
    lua_f_close(l, level);
  }
}
