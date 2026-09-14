//! Node: `cxx:Macro:Luau.VM:VM/src/lvmexecute.cpp:79:VM_INTERRUPT`
//! Source: `VM/src/lvmexecute.cpp:79-91` (hand-ported)
//!
//! C++ `goto exit` becomes `return` — the macro only expands inside
//! `luau_execute_impl`, whose `exit:` label is the function end.

#[macro_export]
macro_rules! VM_INTERRUPT {
  ($l:expr, $pc:expr, $base:expr) => {{
    let interrupt = (*(*$l).global).cb.interrupt;
    if let Some(interrupt) = interrupt {
      // the interrupt hook is called right before we advance pc
      $crate::macros::vm_protect::vm_protect!($l, $pc, $base, {
        (*(*$l).ci).savedpc = (*(*$l).ci).savedpc.add(1);
        interrupt($l, -1);
      });
      if (*$l).status != 0 {
        (*(*$l).ci).savedpc = (*(*$l).ci).savedpc.sub(1);
        return;
      }
    }
  }};
}

pub use VM_INTERRUPT;
