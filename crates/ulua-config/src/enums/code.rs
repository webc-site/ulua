use strum::{Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Default,
  FromRepr,
  IntoStaticStr,
  Display,
  EnumString,
  EnumIter,
)]
#[repr(i32)]
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
  /// 该代码在 `LintOptions::warning_mask` 中的位，对应 C++ `1ull << code`。
  #[inline]
  pub const fn mask_bit(self) -> u64 {
    1u64 << (self as i32)
  }
}
