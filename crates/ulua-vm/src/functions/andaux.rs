use crate::{
  functions::bitfold::bitfold, records::lua_state::LuaState, type_aliases::b_uint::BUint,
};

/// AND 折叠（cpp lbitlib.cpp andaux: `unsigned r = ~0u`）：全一同元并入 [`bitfold`]。
///
/// # Safety
/// 同 [`bitfold`]。
pub(crate) unsafe fn andaux(l: *mut LuaState) -> BUint {
  // Safety: 契约同 bitfold——`l` 为存活调用帧，实参按索引可读
  unsafe { bitfold(l, !0, |a, b| a & b) }
}
