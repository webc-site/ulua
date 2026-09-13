use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, where_clause_needed::WhereClauseNeeded};

impl ErrorConverter {
  pub fn operator_call_63(&self, e: &WhereClauseNeeded) -> String {
    let ty = format!("{:?}", e.ty);
    String::from("Type function instance ")
      + &ty
      + " depends on generic function parameters but does not appear in the function signature; this construct cannot be type-checked at this time"
  }
}
