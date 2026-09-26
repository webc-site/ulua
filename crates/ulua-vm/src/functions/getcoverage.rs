use core::{
  ffi::c_void,
  ptr::{null, write_bytes},
};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{luau_assert::LUAU_ASSERT, luau_insn_e::luau_insn_e, luau_insn_op::luau_insn_op},
};

use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  macros::getstr::getstr,
  records::proto::Proto,
  type_aliases::lua_coverage::LuaCoverage,
};

/// # Safety
/// `l` 必须指向存活 `lua_State` 且所查询的调用帧/Proto/输出记录按约定存活可写。
pub(crate) unsafe fn getcoverage(
  p: *mut Proto,
  depth: i32,
  buffer: *mut i32,
  size: usize,
  context: *mut c_void,
  callback: LuaCoverage,
) {
  // Safety: 契约保证 `buffer..buffer+size` 与指令位图字节数匹配且可写，p 存活时按 pc 索引回写不越出 buffer
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

      let line = lua_g_getline(p, i as i32);
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
