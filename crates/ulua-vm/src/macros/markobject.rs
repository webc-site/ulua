//! Source: `VM/src/lgc.cpp`

#[macro_export]
macro_rules! markobject {
  ($g:expr, $t:expr) => {
    // lgc.cpp:149 — obj2gco on a typed GC pointer is a plain cast in C
    if $crate::macros::iswhite::iswhite!($t as *mut $crate::records::gc_object::GCObject) {
      $crate::functions::reallymarkobject::reallymarkobject(
        $g,
        $t as *mut $crate::records::gc_object::GCObject,
      );
    }
  };
}

pub use markobject;
