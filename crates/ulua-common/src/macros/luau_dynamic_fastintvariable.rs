//! `LUAU_DYNAMIC_FASTINTVARIABLE(flag, def)` — defines a *dynamic* int FastFlag.
//! Reference: `luau/Common/include/Luau/Common.h`. cpp 的 `dynamic` 位在 Rust 侧
//! 无消费者（唯一的读者是 cpp 测试的 `--list-fflags` 打印），故不落字段：动态性
//! 由旗标所在模块 `dfint` 表达，见 [`crate::records::f_value`] 的偏差清单。
//! See [`crate::macros::luau_fastflagvariable`] for the namespace/`pub static`
//! design; reads are `crate::dfint::flag.get()`.

#[macro_export]
macro_rules! LUAU_DYNAMIC_FASTINTVARIABLE {
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
