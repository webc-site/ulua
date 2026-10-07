use crate::{
  functions::bitfold::bitfold, records::lua_state::LuaState, type_aliases::b_uint::BUint,
};

/// AND 折叠（cpp lbitlib.cpp andaux: `unsigned r = ~0u`）：全一同元并入 [`bitfold`]。
///
/// 调用序契约（正确性，非内存安全）：同 [`bitfold`]——以 binary32 C 函数约定被调。
pub(crate) fn andaux(l: &mut LuaState) -> BUint {
  bitfold(l, !0, |a, b| a & b)
}
