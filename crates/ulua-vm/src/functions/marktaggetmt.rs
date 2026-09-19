//! Source: `VM/src/lgc.cpp:870-877` `marktaggetmt`（hand-ported fix）
//!
//! 为每个 userdata tag 补标 `global_State::udatamt` 中挂存的元表。
//! 这些元表是 GC 根，缺失补标会导致弱引用场景下元表被 sweep 回收（悬垂指针）。

use crate::{macros::markobject::markobject, records::global_state::global_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn marktaggetmt(g: *mut global_State) {
  unsafe {
    for &mt in (*g).udatamt.iter() {
      if !mt.is_null() {
        markobject!(g, mt);
      }
    }
  }
}
