use crate::{enums::code::Code, records::lint_warning::LintWarning};

impl LintWarning {
  pub fn get_name(code: Code) -> &'static str {
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

    // 越界判别值（cpp 以 LUAU_ASSERT 拦截）在此退化为 "Unknown"，不留 UB。
    K_WARNING_NAMES
      .get(code as usize)
      .copied()
      .unwrap_or("Unknown")
  }
}
