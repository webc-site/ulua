//! Source: `VM/src/ldo.h` — #define restoreci(l, n) ((CallInfo*)((char*)l->base_ci + (n)))
//! (hand-fixed alongside saveci)

#[macro_export]
macro_rules! restoreci {
  ($l:expr, $n:expr) => {
    (((*$l).base_ci as *mut u8).offset($n as isize) as *mut $crate::records::call_info::CallInfo)
  };
}

pub use restoreci;
