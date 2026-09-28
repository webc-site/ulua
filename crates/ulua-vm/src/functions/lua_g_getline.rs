use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::proto::Proto;

/// # Safety
/// `p` 须指向存活且已 finalize 的 `Proto`，`pc` 满足 `0 <= pc < sizecode`（cpp ldebug.cpp:449，
/// 断言为 debug 兜底）：`abslineinfo` 按 `pc >> linegaplog2` 索引、`lineinfo` 按 `pc` 索引，
/// 两处读取均以该上界落在分配数组界内。
pub unsafe fn lua_g_getline(p: *mut Proto, pc: i32) -> i32 {
  unsafe {
    LUAU_ASSERT!(pc >= 0 && pc < (*p).sizecode);

    if (*p).lineinfo.is_null() {
      return 0;
    }

    let abs_index = (pc >> (*p).linegaplog2) as usize;
    let line_index = pc as usize;

    let abs_line = *((*p).abslineinfo.add(abs_index));
    let line_offset = *((*p).lineinfo.add(line_index)) as i32;

    abs_line + line_offset
  }
}
