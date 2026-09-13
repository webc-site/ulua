use core::ffi::c_int;

use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_op::LUAU_INSN_OP};

use crate::{
  functions::{c_slice, lua_g_getline::luaG_getline},
  type_aliases::proto::Proto,
};

pub(crate) unsafe fn getnextline(p: *mut Proto, line: c_int) -> c_int {
  unsafe {
    let mut closest = -1;

    if !(*p).lineinfo.is_null() {
      for (i, &insn) in c_slice((*p).code, (*p).sizecode as usize)
        .iter()
        .enumerate()
      {
        if LUAU_INSN_OP(insn) == LuauOpcode::LOP_PREPVARARGS as u32 {
          continue;
        }

        let candidate = luaG_getline(p, i as c_int);

        if candidate == line {
          return line;
        }

        if candidate > line && (closest == -1 || candidate < closest) {
          closest = candidate;
        }
      }
    }

    for &sub in c_slice((*p).p, (*p).sizep as usize) {
      let candidate = getnextline(sub, line);

      if candidate == line {
        return line;
      }

      if candidate > line && (closest == -1 || candidate < closest) {
        closest = candidate;
      }
    }

    closest
  }
}
