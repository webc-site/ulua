use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, swapped_generic_type_parameter::SwappedGenericTypeParameter,
};

impl ErrorConverter {
  pub fn operator_call_38(&self, e: &SwappedGenericTypeParameter) -> String {
    match e.kind {
      SwappedGenericTypeParameter::TYPE => {
        let mut result = String::from("Variadic type parameter '");
        result.push_str(&e.name);
        result.push_str("...' is used as a regular generic type; consider changing '");
        result.push_str(&e.name);
        result.push_str("...' to '");
        result.push_str(&e.name);
        result.push_str("' in the generic argument list");
        result
      }
      SwappedGenericTypeParameter::PACK => {
        let mut result = String::from("Generic type '");
        result.push_str(&e.name);
        result.push_str("' is used as a variadic type parameter; consider changing '");
        result.push_str(&e.name);
        result.push_str("' to '");
        result.push_str(&e.name);
        result.push_str("...' in the generic argument list");
        result
      }
    }
  }
}
