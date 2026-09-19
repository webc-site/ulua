use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{error_converter::ErrorConverter, generic_bounds_mismatch::GenericBoundsMismatch},
};

impl ErrorConverter {
  pub fn operator_call_9(&self, e: &GenericBoundsMismatch) -> String {
    let mut lower_bounds = String::new();
    for (i, bound) in e.lower_bounds.iter().enumerate() {
      if i > 0 {
        lower_bounds.push_str(" | ");
      }
      lower_bounds.push_str(&to_string_type_id(*bound));
    }

    let mut upper_bounds = String::new();
    for (i, bound) in e.upper_bounds.iter().enumerate() {
      if i > 0 {
        upper_bounds.push_str(" & ");
      }
      upper_bounds.push_str(&to_string_type_id(*bound));
    }

    format!(
      "No valid instantiation could be inferred for generic type parameter {}. It was expected to be at least:\n\t{}\nand at most:\n\t{}\nbut these types are not compatible with one another.",
      e.generic_name, lower_bounds, upper_bounds
    )
  }
}
