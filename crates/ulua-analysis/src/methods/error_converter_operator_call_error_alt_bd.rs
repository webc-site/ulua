use alloc::string::{String, ToString};

use crate::records::{
  error_converter::ErrorConverter, generic_type_pack_count_mismatch::GenericTypePackCountMismatch,
};

impl ErrorConverter {
  pub fn operator_call_11(&self, e: &GenericTypePackCountMismatch) -> String {
    let mut result = String::from("Different number of generic type pack parameters: subtype had ");
    result.push_str(&e.sub_ty_generic_pack_count.to_string());
    result.push_str(", supertype had ");
    result.push_str(&e.super_ty_generic_pack_count.to_string());
    result.push('.');
    result
  }
}
