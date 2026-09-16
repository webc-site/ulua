use core::ffi::c_void;

use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::{
  functions::mk_name_topo_sort_statements_alt_b::mk_name_ast_expr_local,
  records::arc_collector::ArcCollector,
};
impl ArcCollector {
  pub fn visit_ast_expr_local(&mut self, node: *mut c_void) -> bool {
    let node = unsafe { &*(node as *mut AstExprLocal) };
    let name = mk_name_ast_expr_local(node);
    self.add(&name);
    true
  }
}
