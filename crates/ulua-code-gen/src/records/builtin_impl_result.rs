use crate::enums::builtin_impl_type::BuiltinImplType;

/// builtin 快速路径翻译结果（`IrTranslateBuiltins.h:19`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BuiltinImplResult {
  pub r#type: BuiltinImplType,
  pub actual_result_count: i32,
}

impl BuiltinImplResult {
  /// 通用构造器，覆盖零散形态（`UsesFallback`、非 1 结果数等）。
  pub const fn new(r#type: BuiltinImplType, actual_result_count: i32) -> Self {
    Self {
      r#type,
      actual_result_count,
    }
  }

  /// 未实现/参数不符形态（`None, -1`）：builtin 未被快速路径覆盖，
  /// 调用方（`translate_fast_call_n`）据此保留 fallback 慢路径。
  /// 对应 C++ 最常见的 `return {BuiltinImplType::None, -1};`。
  pub const NONE_FALLBACK: Self = Self::new(BuiltinImplType::None, -1);

  /// 完整实现且恰产出 1 个结果（`Full, 1`），对应 C++ `return {BuiltinImplType::Full, 1};`。
  pub const FULL_ONE_RESULT: Self = Self::new(BuiltinImplType::Full, 1);
}

impl Default for BuiltinImplResult {
  fn default() -> Self {
    Self::new(BuiltinImplType::None, 0)
  }
}
