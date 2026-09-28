use crate::{
  functions::stringresizeprotected::stringresizeprotected, records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// 负载判定与减半目标见 `Stringtable::wants_shrink/half_size`。
pub(crate) unsafe fn shrinkbuffers(l: *mut LuaState) {
  unsafe {
    let g = (*l).global;
    if (*g).strt.wants_shrink() {
      let half = (*g).strt.half_size();
      stringresizeprotected(l, half);
    }
  }
}
