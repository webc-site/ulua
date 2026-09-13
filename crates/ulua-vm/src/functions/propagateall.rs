//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:592:propagateall`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:592-600, hand-ported)

use crate::{functions::propagatemark::propagatemark, records::global_state::global_State};

pub(crate) unsafe fn propagateall(g: *mut global_State) -> usize {
  unsafe {
    let mut work: usize = 0;
    while !(*g).gray.is_null() {
      work += propagatemark(g);
    }
    work
  }
}
