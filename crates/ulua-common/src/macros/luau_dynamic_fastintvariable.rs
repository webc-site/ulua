//! `LUAU_DYNAMIC_FASTINTVARIABLE(flag, def)` — defines a *dynamic* int FastFlag
//! (the `dynamic` bit is `true`). Reference: `luau/Common/include/Luau/Common.h`.
//! See [`crate::macros::luau_fastflagvariable`] for the namespace/`pub static`
//! design; reads are `DFInt::flag` -> `crate::DFInt::flag.get()`.

#[macro_export]
macro_rules! LUAU_DYNAMIC_FASTINTVARIABLE {
  ($screaming:ident, $orig:ident, $def:expr) => {
    pub static $screaming: $crate::records::f_value::FValue<i32> =
      $crate::records::f_value::FValue::new(stringify!($orig), $def, true);
  };
  ($flag:ident, $def:expr) => {
    pub static $flag: $crate::records::f_value::FValue<i32> =
      $crate::records::f_value::FValue::new(stringify!($flag), $def, true);
  };
}

pub use LUAU_DYNAMIC_FASTINTVARIABLE;
