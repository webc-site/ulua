use crate::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

/// cpp `getnextbuffersize`（`VM/src/laux.cpp:430-444`）的直译。
///
/// r12-w6 保留 `unsafe fn` 裸 `l` 形判定：唯一消费方 `extendstrbuf` 的 `l` 自 `b.l`
/// 句柄就地折回裸形、可退化为 null（未接线形态与 cpp 等价）——收形 `&mut` 会强制调用点
/// 对可空句柄先重建引用（当场 UB），而本函数实际解引用仅发生在溢出抛错臂（`lua_l_error_l`
/// 裸形契约透传，抛后不返回）。按判例 1（调用方 supplied 裸指针且体内解引用）保留，转手
/// 屏障按 r16-v21 判例。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；溢出分支经 `luaL_error!` 抛出，不返回——走到该臂时
/// `l` 非空且处于可抛错受保护帧。
pub(crate) unsafe fn getnextbuffersize(
  l: *mut LuaState,
  currentsize: usize,
  desiredsize: usize,
) -> usize {
  let newsize = currentsize + currentsize / 2;

  // check for size overflow（cpp laux.cpp:434-436）
  if usize::MAX - desiredsize < currentsize {
    // SAFETY: 契约保证 `L` 为存活调用帧，尺寸溢出路径经 luaL_error! 抛错后不再返回
    unsafe { luaL_error!(&mut *l, "buffer too large") }
  }

  // growth factor might not be enough to satisfy the desired size
  newsize.max(desiredsize)
}
