//! `luaG_onbreak` — returns true when the current instruction at the top
//! call frame is `LOP_BREAK`.
//! C++ source: `VM/src/ldebug.cpp:405`

use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_ops::luau_insn_op};

use crate::records::lua_state::LuaState;

/// 判定顶层帧当前指令是否 `LOP_BREAK`（`luaG_onbreak`）。`l` 以引用传入（存活由类型
/// 保证）：仅当 `ci != base_ci`（有活动帧）时读 `(*ci).func`；判定通过后
/// `(*ci).savedpc` 指向所属 `Proto.code` 内合法指令字是 Lua 帧结构不变量。
/// `ci==base_ci` 或非 Lua 帧时提前返回 false，不触碰上述指针。纯只读，不分配、不抛错。
/// cpp VM/src/ldebug.cpp:405
pub fn lua_g_onbreak(l: &LuaState) -> bool {
  // SAFETY: 帧链判空与闭包 tag 判定在前（提前返回 false），块内只读帧槽与指令字。
  unsafe {
    if l.ci == l.base_ci {
      return false;
    }

    let func = (*l.ci).func;
    let Some(cl) = (*(*func).value.gc).as_closure() else {
      return false;
    };
    if cl.is_c != 0 {
      return false;
    }

    luau_insn_op(*(*l.ci).savedpc) == LuauOpcode::LOP_BREAK as u32
  }
}
