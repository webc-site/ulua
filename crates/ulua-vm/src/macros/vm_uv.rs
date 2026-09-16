//! Node: `cxx:Macro:Luau.VM:VM/src/lvmexecute.cpp:71:VM_UV`
//! Source: `VM/src/lvmexecute.cpp:71` (hand-ported)
//!
//! `uprefs` is the C flexible-array-member idiom (`TValue uprefs[1]` with
//! over-allocation) — indexing the Rust `[TValue; 1]` would PANIC for i >= 1,
//! so the element is reached via pointer arithmetic, exactly like C.

#[macro_export]
macro_rules! VM_UV {
  ($i:expr, $cl:expr) => {{
    let i = $i;
    let cl = $cl;
    ulua_common::LUAU_ASSERT!((i as u32) < ((*cl).nupvalues as u32));
    let l = &mut (*cl).inner.l;
    &mut *l.uprefs.as_mut_ptr().add(i as usize)
  }};
}

pub use VM_UV;
