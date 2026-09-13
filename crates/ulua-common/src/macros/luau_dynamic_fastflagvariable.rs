//! `LUAU_DYNAMIC_FASTFLAGVARIABLE(flag, def)` — defines a *dynamic* bool FastFlag
//! (the `dynamic` bit is `true`). Reference: `luau/Common/include/Luau/Common.h`.
//! See [`crate::macros::luau_fastflagvariable`] for the namespace/`pub static`
//! design; reads are `DFFlag::flag` -> `crate::DFFlag::flag.get()`.

#[macro_export]
macro_rules! LUAU_DYNAMIC_FASTFLAGVARIABLE {
  ($screaming:ident, $orig:ident, $def:expr) => {
    pub static $screaming: $crate::records::f_value::FValue<bool> =
      $crate::records::f_value::FValue::new(stringify!($orig), $def, true);
  };
  ($flag:ident, $def:expr) => {
    pub static $flag: $crate::records::f_value::FValue<bool> =
      $crate::records::f_value::FValue::new(stringify!($flag), $def, true);
  };
}

pub use LUAU_DYNAMIC_FASTFLAGVARIABLE;
