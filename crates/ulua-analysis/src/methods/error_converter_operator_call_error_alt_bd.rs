use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, generic_type_pack_count_mismatch::GenericTypePackCountMismatch,
};

impl ErrorConverter {
  pub fn operator_call_11(&self, e: &GenericTypePackCountMismatch) -> String {
    format!(
      "Different number of generic type pack parameters: subtype had {}, supertype had {}.",
      e.sub_ty_generic_pack_count, e.super_ty_generic_pack_count
    )
  }
}
