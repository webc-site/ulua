use core::{
  ffi::{c_char, c_int},
  mem::size_of,
};

use crate::{records::lua_state::lua_State, type_aliases::t_value::TValue};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn stacklimitreached(l: *mut lua_State, n: c_int) -> bool {
  unsafe {
    let stack_last = (*l).stack_last as *mut c_char;
    let top = (*l).top as *mut c_char;
    // C++ does SIGNED pointer subtraction (`ptrdiff_t`): once `top` passes
    // `stack_last`, the difference goes negative → the limit is reached. The
    // original `as usize` subtraction underflowed when `top > stack_last` —
    // a panic with overflow-checks (fuzz build), and in release it wraps to a
    // huge value, wrongly reporting "limit NOT reached". Compute it signed to
    // match C++. (Found by the `splice` fuzz target — a deeply recursive
    // program pushes `top` past `stack_last`.)
    let diff = (stack_last as isize) - (top as isize);
    let threshold = (n as isize) * (size_of::<TValue>() as isize);
    diff <= threshold
  }
}
