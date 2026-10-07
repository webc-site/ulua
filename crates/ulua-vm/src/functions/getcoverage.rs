use core::{ffi::c_void, ptr::null};

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

/// 覆盖率回写（cpp ldebug.cpp:564 `getcoverage`）：按原型 `LOP_COVERAGE` 指令把每行命中数
/// 折进 `buffer`（长度即行号上界），逐原型深度优先回调宿主一次。出参裸缓冲已折叠为
/// `&mut [i32]`（§3 出参惯用化）：所有权在 `lua_getcoverage` 帧内闭合，仅回调边界暂交裸指针。
///
/// 本函数为 safe 包装：其运行期开放边界（`callback`/`context` 型别匹配、回调不得留存缓冲指针）
/// 由 [`crate::functions::lua_getcoverage::lua_getcoverage`] 的 `# Safety` 在公开边界收口；
/// `p`（含递归子原型）与 `code`/`sizecode`/`lineinfo` 的存活自洽由 loader/编译器不变量保证。
pub(crate) fn getcoverage(
  p: &Proto,
  depth: i32,
  buffer: &mut [i32],
  context: *mut c_void,
  callback: LuaCoverage,
) {
  // 行初值 -1（cpp `memset(buffer, -1, size * sizeof(int))`）
  buffer.fill(-1);

  // SAFETY: 契约（见函数 doc）保证 `code` 覆盖 `sizecode` 项且在本轮只读消费期内存活。
  let code = unsafe { c_slice(p.code, p.sizecode as usize) };

  for (i, &insn) in code.iter().enumerate() {
    if luau_insn_op(insn) != LuauOpcode::LOP_COVERAGE as u32 {
      continue;
    }

    let line = lua_g_getline(p, i as i32);
    let hits = luau_insn_e(insn);

    LUAU_ASSERT!((line as usize) < buffer.len());
    // 越界行号（上游不变量已破坏）经 `get_mut` 折叠为跳过，替代 cpp 裸下标 UB
    if let Some(slot) = buffer.get_mut(line as usize) {
      *slot = (*slot).max(hits);
    }
  }

  let debugname = if p.debugname.is_null() {
    null()
  } else {
    // SAFETY: 契约保证 `debugname` 非空即指向存活 `tstring`，`getstr` 仅取串体首址（只读）。
    unsafe { getstr(p.debugname) }
  };
  let linedefined = p.linedefined;

  if let Some(cb) = callback {
    // SAFETY: C-ABI 宿主回调属运行期开放边界——`lua_getcoverage` 的 `# Safety` 收口
    // `context` 与回调型别匹配、回调在本调用窗口内可解引用 `buffer` 首址但不得留存；
    // `buffer` 为本帧独占可变区，借用半径止于本次调用。
    unsafe {
      cb(
        context,
        debugname,
        linedefined,
        depth,
        buffer.as_mut_ptr(),
        buffer.len(),
      )
    };
  }

  // SAFETY: 契约保证子原型数组按 `sizep` 存活（递归下传，同函数 doc）。
  let subs = unsafe { c_slice(p.p, p.sizep as usize) };
  for &sub in subs {
    // SAFETY: 同上契约，`sub` 为数组内存活 `Proto` 句柄，降共享引用递归下传。
    getcoverage(unsafe { &*sub }, depth + 1, buffer, context, callback);
  }
}
