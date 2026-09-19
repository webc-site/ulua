use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_op::luau_insn_op};

use crate::{
  functions::{c_slice, lua_g_getline::luaG_getline},
  type_aliases::proto::Proto,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn getnextline(p: *mut Proto, line: i32) -> i32 {
  unsafe {
    let mut closest = -1;

    if !(*p).lineinfo.is_null() {
      for (i, &insn) in c_slice((*p).code, (*p).sizecode as usize)
        .iter()
        .enumerate()
      {
        if luau_insn_op(insn) == LuauOpcode::LOP_PREPVARARGS as u32 {
          continue;
        }

        let candidate = luaG_getline(p, i as i32);

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
