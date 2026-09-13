use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{
    dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe,
    error_converter::ErrorConverter,
  },
};
impl ErrorConverter {
  pub fn operator_call_7(&self, e: &DynamicPropertyLookupOnExternTypesUnsafe) -> String {
    "Attempting a dynamic property access on type '".to_owned()
      + &to_string_type_id(e.ty)
      + "' is unsafe and may cause exceptions at runtime"
  }
}
