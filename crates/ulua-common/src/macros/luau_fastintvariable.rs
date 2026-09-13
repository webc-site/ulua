//! `LUAU_FASTINTVARIABLE(flag, def)` — defines a static (non-dynamic) int
//! FastFlag. Reference: `luau/Common/include/Luau/Common.h`. See
//! [`crate::macros::luau_fastflagvariable`] for the namespace/`pub static` design.

#[macro_export]
macro_rules! LUAU_FASTINTVARIABLE {
  ($screaming:ident, $orig:ident, $def:expr) => {
    pub static $screaming: $crate::records::f_value::FValue<i32> =
      $crate::records::f_value::FValue::new(stringify!($orig), $def, false);
  };
  ($flag:ident, $def:expr) => {
    pub static $flag: $crate::records::f_value::FValue<i32> =
      $crate::records::f_value::FValue::new(stringify!($flag), $def, false);
  };
}

pub use LUAU_FASTINTVARIABLE;
