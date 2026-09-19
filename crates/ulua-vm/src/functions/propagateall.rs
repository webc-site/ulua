//! Source: `VM/src/lgc.cpp` (lgc.cpp:592-600, hand-ported)

use crate::{functions::propagatemark::propagatemark, records::global_state::global_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn propagateall(g: *mut global_State) -> usize {
  unsafe {
    let mut work: usize = 0;
    while !(*g).gray.is_null() {
      work += propagatemark(g);
    }
    work
  }
}
