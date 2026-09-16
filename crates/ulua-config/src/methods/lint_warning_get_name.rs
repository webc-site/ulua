use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{enums::code::Code, records::lint_warning::LintWarning};

impl LintWarning {
  pub fn get_name(code: Code) -> &'static str {
    LUAU_ASSERT!((code as usize) < Code::COUNT);

    const K_WARNING_NAMES: [&str; Code::COUNT] = [
      "Unknown",
      "UnknownGlobal",
      "DeprecatedGlobal",
      "GlobalUsedAsLocal",
      "LocalShadow",
      "SameLineStatement",
      "MultiLineStatement",
      "LocalUnused",
      "FunctionUnused",
      "ImportUnused",
      "BuiltinGlobalWrite",
      "PlaceholderRead",
      "UnreachableCode",
      "UnknownType",
      "ForRange",
      "UnbalancedAssignment",
      "ImplicitReturn",
      "DuplicateLocal",
      "FormatString",
      "TableLiteral",
      "UninitializedLocal",
      "DuplicateFunction",
      "DeprecatedApi",
      "TableOperations",
      "DuplicateCondition",
      "MisleadingAndOr",
      "CommentDirective",
      "IntegerParsing",
      "ComparisonPrecedence",
      "RedundantNativeAttribute",
    ];

    // Code 判别值恒在 0..=29，数组长度即 COUNT，索引必然安全
    unsafe { K_WARNING_NAMES.get_unchecked(code as usize) }
  }
}
