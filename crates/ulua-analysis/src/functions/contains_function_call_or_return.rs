use ulua_ast::{records::ast_stat::AstStat, visit::ast_stat_visit};

use crate::records::contains_function_call::ContainsFunctionCall;

pub fn contains_function_call_or_return(stat: &AstStat) -> bool {
  let mut cfc = ContainsFunctionCall::new(true);
  unsafe {
    let stat_ptr = stat as *const AstStat as *mut AstStat;
    ast_stat_visit(stat_ptr, &mut cfc);
  }
  cfc.result
}
