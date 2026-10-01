// VM/src/lfunc.h:14 — #define upisopen(up) ((up)->v != &(up)->u.value)
#[macro_export]
macro_rules! upisopen {
  ($up:expr) => {
    (*$up).v != core::ptr::addr_of!((*$up).u.value).cast_mut()
  };
}

pub use upisopen;
