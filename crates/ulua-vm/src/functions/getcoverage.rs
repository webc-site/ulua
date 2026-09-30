use core::{
  ffi::c_void,
  ptr::{null, write_bytes},
};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_assert::LUAU_ASSERT,
    luau_insn_ops::{luau_insn_e, luau_insn_op},
  },
};

use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  macros::getstr::getstr,
  records::proto::Proto,
  type_aliases::lua_coverage::LuaCoverage,
};

/// # Safety
/// `p` 须为指向存活 `Proto` 的共享引用（子原型数组元素亦须存活，递归下传）；`buffer..buffer+size`
/// 为调用方可写输出区且长度覆盖行号上界；`callback` 型别约定与 `context` 匹配。本函数只读 Proto，
/// 写操作仅落在 `buffer`。
pub(crate) unsafe fn getcoverage(
  p: &Proto,
  depth: i32,
  buffer: *mut i32,
  size: usize,
  context: *mut c_void,
  callback: LuaCoverage,
) {
  // SAFETY: 契约保证 `buffer..buffer+size` 与指令位图字节数匹配且可写，p 存活时按 pc 索引回写不越出 buffer
  unsafe {
    write_bytes(buffer, 0xFF, size);

    for (i, &insn) in c_slice(p.code, p.sizecode as usize).iter().enumerate() {
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

    let debugname = if !p.debugname.is_null() {
      getstr(p.debugname)
    } else {
      null()
    };
    let linedefined = p.linedefined;

    if let Some(cb) = callback {
      cb(context, debugname, linedefined, depth, buffer, size);
    }

    for &sub in c_slice(p.p, p.sizep as usize) {
      getcoverage(&*sub, depth + 1, buffer, size, context, callback);
    }
  }
}
