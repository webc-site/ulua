use core::ffi::{c_int, c_uchar};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{luau_assert::LUAU_ASSERT, luau_insn_op::LUAU_INSN_OP},
};

use crate::{
  functions::{c_slice, c_slice_mut, lua_g_getline::luaG_getline},
  macros::lua_m_newarray::luaM_newarray,
  records::proto::Proto,
  type_aliases::{instruction::Instruction, lua_state::lua_State},
};

/// C++ `void luaG_breakpoint(lua_State* l, Proto* p, int line, bool enable)`.
pub(crate) unsafe fn lua_g_breakpoint(l: *mut lua_State, p: *mut Proto, line: c_int, enable: bool) {
  unsafe {
    let ondisable = (*(*l).global).ecb.disable;

    if !(*p).lineinfo.is_null() && (ondisable.is_some() || (*p).execdata.is_null()) {
      for (i, &insn) in c_slice((*p).code, (*p).sizecode as usize)
        .iter()
        .enumerate()
      {
        if LUAU_INSN_OP(insn) == LuauOpcode::LOP_PREPVARARGS as u32 {
          continue;
        }

        if luaG_getline(p, i as c_int) != line {
          continue;
        }

        if (*p).debuginsn.is_null() {
          (*p).debuginsn = luaM_newarray!(l, (*p).sizecode, c_uchar, (*p).hdr.memcat);
          // debuginsn 快照：源 code 切片 zip 目标，单次遍历
          for (d, &snap) in c_slice_mut((*p).debuginsn, (*p).sizecode as usize)
            .iter_mut()
            .zip(c_slice((*p).code, (*p).sizecode as usize))
          {
            *d = LUAU_INSN_OP(snap) as c_uchar;
          }
        }

        let op = if enable {
          LuauOpcode::LOP_BREAK as u32
        } else {
          *((*p).debuginsn.add(i)) as u32
        };

        (*(*p).code.add(i)) &= !(0xff as Instruction);
        (*(*p).code.add(i)) |= op as Instruction;
        LUAU_ASSERT!(LUAU_INSN_OP(*(*p).code.add(i)) == op);

        if enable
          && !(*p).execdata.is_null()
          && let Some(ondisable) = ondisable
        {
          ondisable(l, p);
        }

        break;
      }
    }

    for &sub in c_slice((*p).p, (*p).sizep as usize) {
      lua_g_breakpoint(l, sub, line, enable);
    }
  }
}
