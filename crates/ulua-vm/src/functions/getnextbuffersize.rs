use crate::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

/// cpp `getnextbuffersize`（`VM/src/laux.cpp:430-444`）的直译。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；溢出分支经 `luaL_error!` 抛出，不返回。
pub(crate) unsafe fn getnextbuffersize(
  l: *mut LuaState,
  currentsize: usize,
  desiredsize: usize,
) -> usize {
  let newsize = currentsize + currentsize / 2;

  // check for size overflow（cpp laux.cpp:434-436）
  if usize::MAX - desiredsize < currentsize {
    // Safety: 契约保证 `L` 为存活调用帧，尺寸溢出路径经 luaL_error! 抛错后不再返回
    unsafe { luaL_error!(l, "buffer too large") }
  }

  // growth factor might not be enough to satisfy the desired size
  newsize.max(desiredsize)
}
