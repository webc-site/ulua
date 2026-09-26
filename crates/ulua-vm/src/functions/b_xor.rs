use crate::{
  functions::bitfold::bit_fold_push, records::lua_state::LuaState, type_aliases::b_uint::BUint,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe extern "C-unwind" fn b_xor(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 指向存活 LuaState，XOR 折叠单位元为 0，折叠后纯数值压栈（栈顶已留余量）
  unsafe { bit_fold_push(l, 0 as BUint, |a, b| a ^ b) }
}
