//! `luaG_onbreak` — returns true when the current instruction at the top
//! call frame is `LOP_BREAK`.
//! C++ source: `VM/src/ldebug.cpp:405`

use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_op::luau_insn_op};

use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState`：仅当 `(*l).ci != (*l).base_ci`（有活动帧）时读 `(*ci).func`（须为 Function 且
/// 非 C 闭包，其 `value.gc->cl` 存活）；判定通过后 `(*ci).savedpc` 须指向其所属 `Proto.code` 内的合法指令字方可解引用。
/// `ci==base_ci` 或非 Lua 帧时提前返回 false，不触碰上述指针。纯只读，不分配、不抛错。
/// cpp VM/src/ldebug.cpp:405
pub unsafe fn lua_g_onbreak(l: *mut LuaState) -> bool {
  unsafe {
    if (*l).ci == (*l).base_ci {
      return false;
    }

    let func = (*(*l).ci).func;
    let Some(cl) = (*(*func).value.gc).as_closure() else {
      return false;
    };
    if cl.is_c != 0 {
      return false;
    }

    luau_insn_op(*(*(*l).ci).savedpc) == LuauOpcode::LOP_BREAK as u32
  }
}
