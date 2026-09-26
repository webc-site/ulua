//! 分析层「魔法名字面量」单点表。
//!
//! cpp 把 `%error-id%`、`typeof`、`_luau_print` 这类由词法器/内建类型注入的特殊名字在
//! TypeInfer.cpp、ConstraintGenerator.cpp、TypeUtils.h 等处逐字抄写多遍，Rust 直译于是
//! 继承了 6~7 处手写的 `name.as_bytes() == b"..." || …` 与 9 处 `== "_luau_*"` 短路链。
//! 本模块把这些字面量收为编译期常量、把同族判定合并为一枚 `const fn`：常量值与原文
//! 逐字相同，故比较站点与打印/报错文案输出一字不变。

/// cpp `kParseNameError`（`ParseResult.h`）：语法错误节点的占位名。AstName 由词法器
/// intern，必为合法 ASCII/UTF-8，用字节串比较即可，免去 `CStr` 的 unsafe。
pub(crate) const K_ERROR_ID: &[u8] = b"%error-id%";

/// `typeof`：内建类型查询运算符，不是合法的 type alias / 类型函数名。
pub(crate) const K_TYPEOF: &[u8] = b"typeof";

// —— cpp `TypeUtils.h` 的 `kLuau*` 魔法类型名（由 `DebugLuauMagicTypes` 开关放行）——
// 常量值与原文逐字相同，故比较站点与打印/报错文案的输出保持不变。

/// cpp `kLuauPrint`：把类型打印到 `set_print_line` 注册的回调上。
pub(crate) const LUAU_PRINT: &str = "_luau_print";

/// cpp `kLuauIce`：触发一次内部错误（ICE）报告，用于排障。
pub(crate) const LUAU_ICE: &str = "_luau_ice";

/// cpp `kLuauBlockedType`：直接产出一个 `BlockedType` 占位。
pub(crate) const LUAU_BLOCKED_TYPE: &str = "_luau_blocked_type";

/// cpp `kLuauForceConstraintSolvingIncomplete`：强制上报「求解不完整」错误。
pub(crate) const LUAU_FORCE_CONSTRAINT_SOLVING_INCOMPLETE: &str =
  "_luau_force_constraint_solving_incomplete";

/// `&[u8]` 等值的 `const fn` 版本：切片的 `PartialEq` 不能在常量上下文调用，
/// 故手写等长 + 逐字节比较，语义与 `==` 完全一致。
const fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

/// 保留的类型别名名（`%error-id%` 与 `typeof`）：这类名字不参与别名的原型/绑定流程。
/// 对应 cpp `prototype()` / `check()` / `checkBlockTypeAliases()` /
/// `prototypeTypeDefinitions()` 各抄了一遍的同一条判定；入参取 `AstName::as_bytes()`。
pub(crate) const fn is_reserved_type_alias_name(name: &[u8]) -> bool {
  bytes_eq(name, K_ERROR_ID) || bytes_eq(name, K_TYPEOF)
}
