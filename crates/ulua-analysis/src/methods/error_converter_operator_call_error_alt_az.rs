use alloc::string::String;

use crate::{
  enums::reason::Reason,
  functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut,
  records::{
    cannot_assign_to_never::CannotAssignToNever, error_converter::ErrorConverter,
    to_string_options::ToStringOptions,
  },
};
impl ErrorConverter {
  pub fn operator_call_3(&self, e: &CannotAssignToNever) -> String {
    let opts = ToStringOptions::default();
    let rhs_type_str = to_string_type_id_to_string_options_mut(e.rhs_type(), opts);
    let mut result =
      String::from("Cannot assign a value of type ") + &rhs_type_str + " to a field of type never";

    if e.reason() == Reason::PropertyNarrowed && !e.cause().is_empty() {
      result.push_str("\ncaused by the property being given the following incompatible types:\n");
      for ty in e.cause() {
        let opts = ToStringOptions::default();
        let ty_str = to_string_type_id_to_string_options_mut(*ty, opts);
        result.push_str("    ");
        result.push_str(&ty_str);
        result.push('\n');
      }
      result.push_str("There are no values that could safely satisfy all of these types at once.");
    }

    result
  }
}
