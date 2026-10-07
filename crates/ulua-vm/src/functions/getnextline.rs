use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_insn_ops::luau_insn_op};

use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  records::proto::Proto,
};

/// 查 `>= line` 的最近行号（cpp `ldebug.cpp:507`），无则 -1；本函数纯读，不写 Proto。
///
/// 契约：`p` 须为指向存活 `Proto` 的共享引用：当 `p.lineinfo` 非空时 `p.code[0..sizecode]` 须为
/// 有效指令数组，且子原型数组 `p.p[0..sizep]` 每个非空元素须为存活 `Proto`（递归下传，空槽跳过）；
/// `line` 为查询行号。
pub(crate) fn getnextline(p: &Proto, line: i32) -> i32 {
  let mut closest = -1;

  if !p.lineinfo.is_null() {
    // SAFETY: 契约保证 `lineinfo` 非空时 `code[0..sizecode]` 为有效指令数组；只读扫描，
    // 借用半径只覆盖本循环。
    for (i, &insn) in unsafe { c_slice(p.code, p.sizecode as usize) }
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

  // SAFETY: 契约保证 `p.p[0..sizep]` 各元素为存活 `Proto` 的非拥有共享句柄，空槽经 `as_ref` 折叠为
  // None 跳过；递归仅以只读借用下传，不写、不回收，别名只读不冲突。借用半径只覆盖本循环。
  let subs = unsafe { c_slice(p.p, p.sizep as usize) };
  for sub in subs.iter().filter_map(|&sub| unsafe { sub.as_ref() }) {
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
