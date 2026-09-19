use core::{
  ffi::{c_int, c_void},
  ptr::{null, write_bytes},
};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{luau_assert::LUAU_ASSERT, luau_insn_e::luau_insn_e, luau_insn_op::luau_insn_op},
};

use crate::{
  functions::{c_slice, lua_g_getline::luaG_getline},
  macros::getstr::getstr,
  records::proto::Proto,
  type_aliases::lua_coverage::LuaCoverage,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn getcoverage(
  p: *mut Proto,
  depth: c_int,
  buffer: *mut c_int,
  size: usize,
  context: *mut c_void,
  callback: LuaCoverage,
) {
  unsafe {
    write_bytes(buffer, 0xFF, size);

    let p_ref = &*p;
    for (i, &insn) in c_slice(p_ref.code, p_ref.sizecode as usize)
      .iter()
      .enumerate()
    {
      if luau_insn_op(insn) != LuauOpcode::LOP_COVERAGE as u32 {
        continue;
      }

      let line = luaG_getline(p, i as c_int);
      let hits = luau_insn_e(insn);

      LUAU_ASSERT!((line as usize) < size);
      let val = *buffer.add(line as usize);
      if val < hits {
        *buffer.add(line as usize) = hits;
      }
    }

    let debugname = if !p_ref.debugname.is_null() {
      getstr(p_ref.debugname)
    } else {
      null()
    };
    let linedefined = p_ref.linedefined;

    if let Some(cb) = callback {
      cb(context, debugname, linedefined, depth, buffer, size);
    }

    for &sub in c_slice(p_ref.p, p_ref.sizep as usize) {
      getcoverage(sub, depth + 1, buffer, size, context, callback);
    }
  }
}
