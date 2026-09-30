//! Source: `VM/src/lgc.cpp` (lgc.cpp:592-600, hand-ported)

use crate::{functions::propagatemark::propagatemark, records::global_state::global_State};

/// # Safety
/// `g` 须指向存活 global_State 且其 `gray` 灰对象链表自洽（每项 `gclist` 可被 `propagatemark` 消费直至清空）；
/// 须在 GC atomic 阶段独占调用，`propagatemark` 会读写对象标记与灰链，不可并发。cpp/VM/src/lgc.cpp:641 propagateall。
pub(crate) unsafe fn propagateall(g: *mut global_State) -> usize {
  unsafe {
    let mut work: usize = 0;
    while !(*g).gray.is_null() {
      work += propagatemark(g);
    }
    work
  }
}
