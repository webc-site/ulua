use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{functions::c_slice, records::proto::Proto};

/// 行号反查（cpp `ldebug.cpp:449`）：`abslineinfo` 按 `pc >> linegaplog2` 索引、`lineinfo` 按 `pc`
/// 索引，两处读取均以 `sizecode` 上界落在分配数组界内。本函数纯读，不写 Proto。
///
/// 契约（`LUAU_ASSERT` 为 debug 兜底；release 下越界下标经切片 `get` 折叠为读 0，不再越出数组）：
/// `pc` 满足 `0 <= pc < sizecode`；`lineinfo` 为空时直接返回 0。
pub fn lua_g_getline(p: &Proto, pc: i32) -> i32 {
  LUAU_ASSERT!(pc >= 0 && pc < p.sizecode);

  if p.lineinfo.is_null() {
    return 0;
  }

  // SAFETY: 契约保证 `lineinfo`/`abslineinfo` 非拥有句柄存活且与 `sizecode` 自洽——loader/编译器
  // 不变量：`lineinfo` 覆盖 `sizecode` 字节，`abslineinfo` 覆盖 `((sizecode - 1) >> linegaplog2) + 1`
  // 项（loadsafe 分配式）。借用半径只覆盖本函数的两次只读取值。
  let (lineinfo, abslineinfo) = unsafe {
    (
      c_slice(p.lineinfo, p.sizecode as usize),
      c_slice(
        p.abslineinfo,
        (((p.sizecode - 1) >> p.linegaplog2) + 1) as usize,
      ),
    )
  };

  let abs_line = abslineinfo
    .get((pc >> p.linegaplog2) as usize)
    .copied()
    .unwrap_or_default();
  let line_offset = lineinfo.get(pc as usize).copied().unwrap_or_default() as i32;

  abs_line + line_offset
}
