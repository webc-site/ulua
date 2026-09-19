use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::enums::code::Code;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LintWarning {
  pub code: Code,
  pub location: Location,
  pub text: String,
}

/// 仅保留被单元测试引用的别名; 其余 code 一律直接用 [`Code`] 变体,
/// 避免对枚举做二次导出。
impl LintWarning {
  pub const CODE_UNKNOWN_GLOBAL: Code = Code::UnknownGlobal;
  pub const CODE_DEPRECATED_GLOBAL: Code = Code::DeprecatedGlobal;
  pub const CODE_LOCAL_SHADOW: Code = Code::LocalShadow;
  pub const CODE_SAME_LINE_STATEMENT: Code = Code::SameLineStatement;
  pub const CODE_LOCAL_UNUSED: Code = Code::LocalUnused;
  pub const CODE_FUNCTION_UNUSED: Code = Code::FunctionUnused;
  pub const CODE_IMPORT_UNUSED: Code = Code::ImportUnused;
  pub const CODE_FOR_RANGE: Code = Code::ForRange;
  pub const CODE_DEPRECATED_API: Code = Code::DeprecatedApi;
  pub const CODE_COUNT: Code = Code::Count;
}
