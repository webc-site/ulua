//! `LUAU_FASTFLAGVARIABLE(flag)` — defines a static (non-dynamic) bool FastFlag.
//! Reference: `luau/Common/include/Luau/Common.h`.
//!
//! C++ expands to `namespace FFlag { FValue<bool> flag(#flag, false, false); }`.
//! Rust modules aren't open like C++ namespaces, so the macro emits a bare
//! `pub static` at the call site (no per-flag `mod`, which would collide when a
//! file defines two flags); the enclosing per-crate `fflag` module supplies the
//! namespace, so reads are `crate::fflag::flag.get()`.

#[macro_export]
macro_rules! LUAU_FASTFLAGVARIABLE {
  ($screaming:ident, $orig:ident) => {
    pub static $screaming: $crate::records::f_value::FValue<bool> =
      $crate::records::f_value::FValue::<bool>::new(stringify!($orig), false);
    // Pascal 别名随定义一并由宏自产（同一对 token，两处永不漂移），
    // 聚合模块只需 `pub use _inner::*;` 一个 glob 门面。
    pub use $screaming as $orig;
  };
  ($flag:ident) => {
    pub static $flag: $crate::records::f_value::FValue<bool> =
      $crate::records::f_value::FValue::<bool>::new(stringify!($flag), false);
  };
}
