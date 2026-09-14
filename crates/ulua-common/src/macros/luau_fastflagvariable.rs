//! `LUAU_FASTFLAGVARIABLE(flag)` — defines a static (non-dynamic) bool FastFlag.
//! Reference: `luau/Common/include/Luau/Common.h`.
//!
//! C++ expands to `namespace FFlag { FValue<bool> flag(#flag, false, false); }`.
//! Rust modules aren't open like C++ namespaces, so the macro emits a bare
//! `pub static` at the call site (no per-flag `mod`, which would collide when a
//! file defines two flags); the enclosing per-crate `FFlag` module supplies the
//! namespace, so reads stay `FFlag::flag` -> `crate::FFlag::flag.get()`.

#[macro_export]
macro_rules! LUAU_FASTFLAGVARIABLE {
  ($screaming:ident, $orig:ident) => {
    pub static $screaming: $crate::records::f_value::FValue<bool> =
      $crate::records::f_value::FValue::new(stringify!($orig), false, false);
  };
  ($flag:ident) => {
    pub static $flag: $crate::records::f_value::FValue<bool> =
      $crate::records::f_value::FValue::new(stringify!($flag), false, false);
  };
}

pub use LUAU_FASTFLAGVARIABLE;
