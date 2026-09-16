use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, only_tables_can_have_methods::OnlyTablesCanHaveMethods,
};

impl ErrorConverter {
  pub fn operator_call_36(&self, e: &OnlyTablesCanHaveMethods) -> String {
    format!("Cannot add method to non-table type '{:?}'", e.table_type)
  }
}
