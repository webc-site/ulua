use crate::{
  functions::stringresizeprotected::stringresizeprotected, records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// 折半循环目标见 `Stringtable::shrink_target`。
pub(crate) unsafe fn shrinkbuffersfull(l: *mut LuaState) {
  unsafe {
    let g = (*l).global;
    let target = (*g).strt.shrink_target();
    if target != (*g).strt.size {
      stringresizeprotected(l, target);
    }
  }
}
