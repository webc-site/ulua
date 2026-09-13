#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum Code {
  #[default]
  Unknown = 0,
  UnknownGlobal = 1,
  DeprecatedGlobal = 2,
  GlobalUsedAsLocal = 3,
  LocalShadow = 4,
  SameLineStatement = 5,
  MultiLineStatement = 6,
  LocalUnused = 7,
  FunctionUnused = 8,
  ImportUnused = 9,
  BuiltinGlobalWrite = 10,
  PlaceholderRead = 11,
  UnreachableCode = 12,
  UnknownType = 13,
  ForRange = 14,
  UnbalancedAssignment = 15,
  ImplicitReturn = 16,
  DuplicateLocal = 17,
  FormatString = 18,
  TableLiteral = 19,
  UninitializedLocal = 20,
  DuplicateFunction = 21,
  DeprecatedApi = 22,
  TableOperations = 23,
  DuplicateCondition = 24,
  MisleadingAndOr = 25,
  CommentDirective = 26,
  IntegerParsing = 27,
  ComparisonPrecedence = 28,
  RedundantNativeAttribute = 29,
  Count = 30,
}

impl Code {
  /// 判别值总数（含 Unknown），对应 C++ `Code__Count`。
  pub const COUNT: usize = (Code::Count as i32) as usize;

  /// 全部 lint 代码，按判别值升序排列，对应 C++ 的枚举区间遍历。
  pub const ALL: [Code; Self::COUNT] = [
    Code::Unknown,
    Code::UnknownGlobal,
    Code::DeprecatedGlobal,
    Code::GlobalUsedAsLocal,
    Code::LocalShadow,
    Code::SameLineStatement,
    Code::MultiLineStatement,
    Code::LocalUnused,
    Code::FunctionUnused,
    Code::ImportUnused,
    Code::BuiltinGlobalWrite,
    Code::PlaceholderRead,
    Code::UnreachableCode,
    Code::UnknownType,
    Code::ForRange,
    Code::UnbalancedAssignment,
    Code::ImplicitReturn,
    Code::DuplicateLocal,
    Code::FormatString,
    Code::TableLiteral,
    Code::UninitializedLocal,
    Code::DuplicateFunction,
    Code::DeprecatedApi,
    Code::TableOperations,
    Code::DuplicateCondition,
    Code::MisleadingAndOr,
    Code::CommentDirective,
    Code::IntegerParsing,
    Code::ComparisonPrecedence,
    Code::RedundantNativeAttribute,
  ];
}
