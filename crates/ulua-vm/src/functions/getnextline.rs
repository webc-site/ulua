use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_ops::luau_insn_op};

use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  records::proto::Proto,
};

/// # Safety
/// `p` 须为存活 `Proto`：当 `(*p).lineinfo` 非空时 `(*p).code[0..sizecode]` 须为有效指令数组，
/// 且子原型数组 `(*p).p[0..sizep]` 每个元素须为存活 `Proto`（递归调用）；`line` 为查询行号。
/// cpp `ldebug.cpp:507`。
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

        let candidate = lua_g_getline(p, i as i32);

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
