//! `LUAU_FASTINTVARIABLE(flag, def)` — defines a static (non-dynamic) int
//! FastFlag. Reference: `luau/Common/include/Luau/Common.h`. See
//! [`crate::macros::luau_fastflagvariable`] for the namespace/`pub static` design.

#[macro_export]
macro_rules! LUAU_FASTINTVARIABLE {
  ($screaming:ident, $orig:ident, $def:expr) => {
    pub static $screaming: $crate::records::f_value::FValue<i32> =
      $crate::records::f_value::FValue::<i32>::new(stringify!($orig), $def);
    // Pascal 别名随定义一并由宏自产（同一对 token，两处永不漂移），
    // 聚合模块只需 `pub use _inner::*;` 一个 glob 门面。
    pub use $screaming as $orig;
  };
  ($flag:ident, $def:expr) => {
    pub static $flag: $crate::records::f_value::FValue<i32> =
      $crate::records::f_value::FValue::<i32>::new(stringify!($flag), $def);
  };
}
