//! Source: `VM/src/lgc.cpp`

#[macro_export]
macro_rules! markvalue {
  ($g:expr, $o:expr) => {
    // lgc.cpp:142 — checkconsistency is a debug no-op in release builds
    if $crate::macros::iscollectable::iscollectable!($o)
      && $crate::macros::iswhite::iswhite!($crate::macros::gcvalue::gcvalue!($o))
    {
      $crate::functions::reallymarkobject::reallymarkobject(
        $g,
        $crate::macros::gcvalue::gcvalue!($o),
      );
    }
  };
}

pub use markvalue;
