//! 配置解析/套用的错误类型。取代 cpp `Config.cpp` 里 `using Error = std::string`
//! 的裸串异常：可枚举的失败为具名变体（`#[error]` 字面量与历史串逐字一致），
//! 参数化/运行期不透明串（`bad_setting` 文案、内存耗尽、load/lua 运行时错误）
//! 经 [`ConfigError::Message`] 原样携带，保证 `Display` 与旧串完全相同。

use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigError {
  /// 非法 `languagemode` 文本。
  #[error("Bad mode \"{mode}\".  Valid options are nocheck, nonstrict, and strict")]
  BadMode { mode: String },

  /// 配置表键既非字符串也非数字。
  #[error("configuration table keys must be strings or numbers")]
  TableKeyNotStringOrNumber,

  /// 配置表值类型非法（字符串/数字/布尔/嵌套表之外）。
  #[error(
    "configuration value for key \"{key}\" must be a string, number, boolean, or nested table"
  )]
  BadTableValue { key: String },

  /// 具名配置键的值类型不符（对应 cpp `configuration value for key "K" must be T`）。
  /// `expected` 为 cpp 原文尾段类型短语，逐字保留以确保 `Display` 与旧串完全一致。
  #[error("configuration value for key \"{key}\" must be {expected}")]
  BadValueForKey { key: String, expected: &'static str },

  /// 配置脚本执行时 yield（暂不支持）。
  #[error("configuration execution cannot yield")]
  CannotYield,

  /// 配置脚本返回值个数不为 1。
  #[error("configuration must return exactly one value")]
  NotExactlyOneValue,

  /// 配置脚本返回值不是表。
  #[error("configuration did not return a table")]
  NotATable,

  /// `luau` 表的键不是字符串。
  #[error("configuration keys in \"luau\" table must be strings")]
  LuauTableKeyNotString,

  /// `lint` 表的键不是字符串。
  #[error("configuration keys in \"lint\" table must be strings")]
  LintTableKeyNotString,

  /// `lint` 表的值不是布尔。
  #[error("configuration values in \"lint\" table must be booleans")]
  LintTableValueNotBoolean,

  /// `aliases` 表的键不是字符串。
  #[error("configuration keys in \"aliases\" table must be strings")]
  AliasTableKeyNotString,

  /// `aliases` 表的值不是字符串。
  #[error("configuration values in \"aliases\" table must be strings")]
  AliasTableValueNotString,

  /// `globals` 数组出现非数字键。
  #[error("configuration array \"globals\" must only have numeric keys")]
  GlobalsKeyNotNumeric,

  /// `globals` 数组的数字键越界。
  #[error("configuration array \"globals\" contains invalid numeric key")]
  GlobalsInvalidNumericKey,

  /// `globals` 数组元素不是字符串。
  #[error("configuration value in \"globals\" table must be a string")]
  GlobalsElementNotString,

  /// 未提供 `AliasOptions` 却要求解析别名。
  #[error("Cannot parse aliases without alias options")]
  MissingAliasOptions,

  /// 不透明错误串：`bad_setting` 文案、内存耗尽、`load` 或 lua 运行时错误、
  /// 参数化文案（如 `configuration value for key \"...\" must be ...`）。
  #[error("{0}")]
  Message(String),
}

/// cpp `configuration value for key "K" must be T` 的尾段类型短语常量，
/// 集中定义避免调用点重复雷同串。
pub(crate) const VAL_STRING: &str = "a string";
pub(crate) const VAL_TABLE: &str = "a table";
pub(crate) const VAL_BOOL: &str = "a boolean";
pub(crate) const VAL_STR_ARRAY: &str = "an array of strings";

impl ConfigError {
  /// 构造 [`ConfigError::BadValueForKey`]：集中拼装
  /// `configuration value for key "..." must be ...`，取代散落的
  /// `Message(String::from("..."))` 样板，`Display` 逐字不变。
  pub(crate) fn bad_value(key: &str, expected: &'static str) -> Self {
    Self::BadValueForKey {
      key: String::from(key),
      expected,
    }
  }
}
