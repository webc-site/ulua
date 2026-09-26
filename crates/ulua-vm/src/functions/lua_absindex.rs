use crate::{
  macros::{api_check::api_check, lua_ispseudo::lua_ispseudo},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活的 `LuaState`（读其 `top`/`base` 指针求当前帧元素数）；`idx` 须为
/// 合法（伪）索引（api_check 断言约束）。
pub(crate) unsafe fn lua_absindex(l: *mut LuaState, idx: i32) -> i32 {
  // Safety: 契约保证 l 存活，top/base 同属一条栈数组，offset_from 合法
  let top_minus_base = unsafe { (*l).top.offset_from((*l).base) };

  api_check!(
    l,
    (idx > 0 && idx <= top_minus_base as i32)
      || (idx < 0 && -idx <= top_minus_base as i32)
      || lua_ispseudo(idx)
  );

  if idx > 0 || lua_ispseudo(idx) {
    idx
  } else {
    // Safety: 同上——l 存活且 top/base 同分配
    (unsafe { (*l).top.offset_from((*l).base) } as i32) + idx + 1
  }
}
