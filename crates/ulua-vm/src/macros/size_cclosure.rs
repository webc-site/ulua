use core::mem::{offset_of, size_of};

use crate::{
  records::closure::{CClosure, Closure},
  type_aliases::t_value::TValue,
};

// VM/src/lfunc.h:7 — #define sizeCclosure(n) (offsetof(Closure, c.upvals) + sizeof(TValue) * (n))
// ClosureInner fields sit at offset 0 of the union; ManuallyDrop is repr(transparent).
pub const fn size_cclosure(n: i32) -> usize {
  offset_of!(Closure, inner) + offset_of!(CClosure, upvals) + size_of::<TValue>() * n as usize
}
