use core::ffi::c_void;

use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::{
  functions::mk_name_topo_sort_statements_alt_c::mk_name_ast_expr_global,
  records::{arc_collector::ArcCollector, identifier::Identifier},
};
impl ArcCollector {
  pub fn visit_ast_expr_global(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstExprGlobal;
    let name: Identifier = unsafe { mk_name_ast_expr_global(&*node) };
    self.add(&name);
    true
  }
}
