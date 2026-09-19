//! `luaG_onbreak` — returns true when the current instruction at the top
//! call frame is `LOP_BREAK`.
//! C++ source: `VM/src/ldebug.cpp:405`

use core::ffi::c_int;

use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_op::luau_insn_op};

use crate::{enums::lua_type::LuaType, records::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_luaG_onbreak"))]
pub unsafe fn lua_g_onbreak(l: *mut lua_State) -> bool {
  unsafe {
    if (*l).ci == (*l).base_ci {
      return false;
    }

    // isLua(ci): ci->func is a function && !clvalue(ci->func)->is_c
    // Inline to avoid the broken ttisfunction! macro.
    let func = (*(*l).ci).func;
    if (*func).tt() != LuaType::Function as c_int {
      return false;
    }
    if (*(*(*func).value.gc).cl).is_c != 0 {
      return false;
    }

    luau_insn_op(*(*(*l).ci).savedpc) == LuauOpcode::LOP_BREAK as u32
  }
}
