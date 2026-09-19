use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{
    cannot_extend_table::{CannotExtendTable, Context},
    error_converter::ErrorConverter,
  },
};

impl ErrorConverter {
  pub fn operator_call_15(&self, e: &CannotExtendTable) -> String {
    let table = to_string_type_id(e.table_type());
    match e.context() {
      Context::Property => format!("Cannot add property '{}' to table '{}'", e.prop(), table),
      Context::Metatable => format!("Cannot add metatable to table '{table}'"),
      Context::Indexer => format!("Cannot add indexer to table '{table}'"),
    }
  }
}
