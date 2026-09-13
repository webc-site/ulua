use core::ffi::c_void;

use ulua_ast::records::ast_stat_function::AstStatFunction;

use crate::{
  functions::mk_name_topo_sort_statements_alt_h::mk_name_ast_stat_function,
  records::arc_collector::ArcCollector,
};
impl ArcCollector {
  pub fn visit_ast_stat_function(&mut self, node: *mut c_void) -> bool {
    let node_ref = unsafe { &*(node as *mut AstStatFunction) };
    let name = mk_name_ast_stat_function(node_ref);
    self.add(&name);
    true
  }
}
