//! Source: `VM/src/ldo.h` — #define restorestack(l, n) ((TValue*)((char*)l->stack + (n)))
//! (hand-fixed: the generated body had a cast-precedence error and unqualified
//! types that break at expansion sites)

#[macro_export]
macro_rules! restorestack {
  ($l:expr, $n:expr) => {
    (((*$l).stack as *mut u8).offset($n as isize) as *mut $crate::type_aliases::t_value::TValue)
  };
}

pub use restorestack;
