//! FastFlag 定义宏一族：`LUAU_FASTFLAGVARIABLE` / `LUAU_FASTINTVARIABLE` /
//! `LUAU_DYNAMIC_FASTFLAGVARIABLE` / `LUAU_DYNAMIC_FASTINTVARIABLE`。
//! `LUAU_FASTFLAGVARIABLE(flag)` — defines a static (non-dynamic) bool FastFlag.
//! Reference: `luau/Common/include/Luau/Common.h`.
//!
//! C++ expands to `namespace FFlag { FValue<bool> flag(#flag, false, false); }`.
//! Rust modules aren't open like C++ namespaces, so the macro emits a bare
//! `pub static` at the call site (no per-flag `mod`, which would collide when a
//! file defines two flags); the enclosing per-crate `fflag` module supplies the
//! namespace, so reads are `crate::fflag::flag.get()`.

/// 四族公开宏的单一展开体（`alias` 臂 = SCREAMING 定义 + Pascal 别名自产，同一对
/// token 两处永不漂移，聚合模块只需 `pub use _inner::*;` 一个 glob 门面；`single`
/// 臂 = 单名旧形）。全部调用点都在本 crate 的 `fflag`/`dfflag`/`fint`/`dfint`
/// 聚合模块内，故 helper 只需 crate 内可见，不进公开 API。
macro_rules! luau_fast_variable {
  (alias $ty:ty, $screaming:ident, $orig:ident, $def:expr) => {
    pub static $screaming: $crate::records::f_value::FValue<$ty> =
      $crate::records::f_value::FValue::<$ty>::new(stringify!($orig), $def);
    pub use $screaming as $orig;
  };
  (single $ty:ty, $flag:ident, $def:expr) => {
    pub static $flag: $crate::records::f_value::FValue<$ty> =
      $crate::records::f_value::FValue::<$ty>::new(stringify!($flag), $def);
  };
}
pub(crate) use luau_fast_variable;

/// 旗标聚合模块的单一清单展开体：`fflag`/`fint`/`dfflag`/`dfint` 四个模块每个
/// flag 只列一次，宏一次展开出 `_inner` 定义模块、`pub use _inner::*` glob 门面
/// 与 `register_flags()` 注册函数——消灭「定义一遍、注册清单再抄一遍」的双写
/// （原形态两处清单失配即静默漏注册，只能人工对账）。
///
/// 语法：每个条目为 `MACRO!(SCREAMING, Pascal[, default])[, version = N];`
/// —— `MACRO` 是本文件四族公开宏之一，`default` 即其第三实参（int 族必填，
/// bool 族省略），`version = N` 对应注册清单里的 `set_version(N)` 调用。
/// 条目间的 `//` 出处注释在宏解析前已剥离，原样保留。
macro_rules! luau_flag_module {
  ($(
    $mac:ident!( $screaming:ident, $orig:ident $(, $def:expr)? ) $(, version = $ver:expr)? ;
  )*) => {
    pub mod _inner {
      $(crate::$mac!($screaming, $orig $(, $def)?);)*
    }

    /// 宏自产的 Pascal 别名（宏展开内含 `pub use X as Y;`）经 glob 一次性再导出；
    /// 别名对与宏定义同 token 生成，一致性由编译器而非人工清单保证。
    pub use _inner::*;

    /// C++ `FValue` ctor 的 `list = this` 自注册对应物：把本模块定义的全部
    /// flag 挂入 per-type 注册表，供 `set_flag_by_name`/`set_all_unless` 按名
    /// 遍历。仅由 `ensure_flags_registered` 的 `OnceLock` 串行调用一次。
    /// 仅本 crate 消费，降 `pub(crate)`。
    pub(crate) fn register_flags() {
      $(
        _inner::$screaming.register();
        $(_inner::$screaming.set_version($ver);)?
      )*
    }
  };
}
pub(crate) use luau_flag_module;

/// `LUAU_FASTFLAGVARIABLE(flag)` — static bool FastFlag（读法 `crate::fflag`）。
/// Reference: `luau/Common/include/Luau/Common.h`；bool 族无显式默认，恒 `false`。
#[macro_export]
macro_rules! LUAU_FASTFLAGVARIABLE {
  ($screaming:ident, $orig:ident) => {
    $crate::macros::fast_flags::luau_fast_variable!(alias bool, $screaming, $orig, false);
  };
  ($flag:ident) => {
    $crate::macros::fast_flags::luau_fast_variable!(single bool, $flag, false);
  };
}

/// `LUAU_FASTINTVARIABLE(flag, def)` — defines a static (non-dynamic) int
/// FastFlag. Reference: `luau/Common/include/Luau/Common.h`. See
/// [`LUAU_FASTFLAGVARIABLE`] for the namespace/`pub static` design.
#[macro_export]
macro_rules! LUAU_FASTINTVARIABLE {
  ($screaming:ident, $orig:ident, $def:expr) => {
    $crate::macros::fast_flags::luau_fast_variable!(alias i32, $screaming, $orig, $def);
  };
  ($flag:ident, $def:expr) => {
    $crate::macros::fast_flags::luau_fast_variable!(single i32, $flag, $def);
  };
}

/// `LUAU_DYNAMIC_FASTFLAGVARIABLE(flag, def)` — defines a *dynamic* bool FastFlag.
/// Reference: `luau/Common/include/Luau/Common.h`. cpp 的 `dynamic` 位在 Rust 侧
/// 无消费者（唯一的读者是 cpp 测试的 `--list-fflags` 打印），故不落字段：动态性
/// 由旗标所在模块 `dfflag` 表达，见 [`crate::records::f_value`] 的偏差清单。
/// See [`LUAU_FASTFLAGVARIABLE`] for the namespace/`pub static`
/// design; reads are `crate::dfflag::flag.get()`.
#[macro_export]
macro_rules! LUAU_DYNAMIC_FASTFLAGVARIABLE {
  ($screaming:ident, $orig:ident, $def:expr) => {
    $crate::macros::fast_flags::luau_fast_variable!(alias bool, $screaming, $orig, $def);
  };
  ($flag:ident, $def:expr) => {
    $crate::macros::fast_flags::luau_fast_variable!(single bool, $flag, $def);
  };
}

/// `LUAU_DYNAMIC_FASTINTVARIABLE(flag, def)` — defines a *dynamic* int FastFlag.
/// Reference: `luau/Common/include/Luau/Common.h`. cpp 的 `dynamic` 位在 Rust 侧
/// 无消费者（唯一的读者是 cpp 测试的 `--list-fflags` 打印），故不落字段：动态性
/// 由旗标所在模块 `dfint` 表达，见 [`crate::records::f_value`] 的偏差清单。
/// See [`LUAU_FASTFLAGVARIABLE`] for the namespace/`pub static`
/// design; reads are `crate::dfint::flag.get()`.
#[macro_export]
macro_rules! LUAU_DYNAMIC_FASTINTVARIABLE {
  ($screaming:ident, $orig:ident, $def:expr) => {
    $crate::macros::fast_flags::luau_fast_variable!(alias i32, $screaming, $orig, $def);
  };
  ($flag:ident, $def:expr) => {
    $crate::macros::fast_flags::luau_fast_variable!(single i32, $flag, $def);
  };
}
