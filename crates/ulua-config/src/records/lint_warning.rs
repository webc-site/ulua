use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::enums::code::Code;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LintWarning {
  pub code: Code,
  pub location: Location,
  pub text: String,
}

impl LintWarning {
  pub const CODE_UNKNOWN: Code = Code::Unknown;
  pub const CODE_UNKNOWN_GLOBAL: Code = Code::UnknownGlobal;
  pub const CODE_DEPRECATED_GLOBAL: Code = Code::DeprecatedGlobal;
  pub const CODE_GLOBAL_USED_AS_LOCAL: Code = Code::GlobalUsedAsLocal;
  pub const CODE_LOCAL_SHADOW: Code = Code::LocalShadow;
  pub const CODE_SAME_LINE_STATEMENT: Code = Code::SameLineStatement;
  pub const CODE_MULTI_LINE_STATEMENT: Code = Code::MultiLineStatement;
  pub const CODE_LOCAL_UNUSED: Code = Code::LocalUnused;
  pub const CODE_FUNCTION_UNUSED: Code = Code::FunctionUnused;
  pub const CODE_IMPORT_UNUSED: Code = Code::ImportUnused;
  pub const CODE_BUILTIN_GLOBAL_WRITE: Code = Code::BuiltinGlobalWrite;
  pub const CODE_PLACEHOLDER_READ: Code = Code::PlaceholderRead;
  pub const CODE_UNREACHABLE_CODE: Code = Code::UnreachableCode;
  pub const CODE_UNKNOWN_TYPE: Code = Code::UnknownType;
  pub const CODE_FOR_RANGE: Code = Code::ForRange;
  pub const CODE_UNBALANCED_ASSIGNMENT: Code = Code::UnbalancedAssignment;
  pub const CODE_IMPLICIT_RETURN: Code = Code::ImplicitReturn;
  pub const CODE_DUPLICATE_LOCAL: Code = Code::DuplicateLocal;
  pub const CODE_FORMAT_STRING: Code = Code::FormatString;
  pub const CODE_TABLE_LITERAL: Code = Code::TableLiteral;
  pub const CODE_UNINITIALIZED_LOCAL: Code = Code::UninitializedLocal;
  pub const CODE_DUPLICATE_FUNCTION: Code = Code::DuplicateFunction;
  pub const CODE_DEPRECATED_API: Code = Code::DeprecatedApi;
  pub const CODE_TABLE_OPERATIONS: Code = Code::TableOperations;
  pub const CODE_DUPLICATE_CONDITION: Code = Code::DuplicateCondition;
  pub const CODE_MISLEADING_AND_OR: Code = Code::MisleadingAndOr;
  pub const CODE_COMMENT_DIRECTIVE: Code = Code::CommentDirective;
  pub const CODE_INTEGER_PARSING: Code = Code::IntegerParsing;
  pub const CODE_COMPARISON_PRECEDENCE: Code = Code::ComparisonPrecedence;
  pub const CODE_REDUNDANT_NATIVE_ATTRIBUTE: Code = Code::RedundantNativeAttribute;
  pub const CODE_COUNT: Code = Code::Count;
}
