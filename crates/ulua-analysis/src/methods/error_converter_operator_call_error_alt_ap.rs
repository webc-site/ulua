use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, pack_where_clause_needed::PackWhereClauseNeeded,
};

impl ErrorConverter {
  pub fn operator_call_49(&self, e: &PackWhereClauseNeeded) -> String {
    let tp = format!("{:?}", e.tp);
    String::from("Type pack function instance ")
      + &tp
      + " depends on generic function parameters but does not appear in the function signature; this construct cannot be type-checked at this time"
  }
}
