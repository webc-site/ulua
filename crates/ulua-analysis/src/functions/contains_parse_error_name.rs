use crate::{
  records::{invalid_name_checker::InvalidNameChecker, type_error::TypeError},
  type_aliases::type_error_data::TypeErrorData,
};

/// C++ `bool containsParseErrorName(const TypeError& error)` (Error.cpp:1458).
pub fn contains_parse_error_name(error: &TypeError) -> bool {
  let checker = InvalidNameChecker::new();
  match &error.data {
    TypeErrorData::UnknownProperty(e) => checker.operator_unknown_property(e),
    TypeErrorData::CannotExtendTable(e) => checker.operator_cannot_extend_table(e),
    TypeErrorData::DuplicateTypeDefinition(e) => checker.operator_duplicate_type_definition(e),
    other => checker.operator_fallback(other),
  }
}
