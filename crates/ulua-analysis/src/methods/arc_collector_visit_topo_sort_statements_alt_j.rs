use core::ffi::c_void;

use ulua_ast::records::ast_type_typeof::AstTypeTypeof;

use crate::{
  functions::mk_name_topo_sort_statements_alt_g::mk_name_ast_expr,
  records::arc_collector::ArcCollector,
};
impl ArcCollector {
  pub fn visit_ast_type_typeof(&mut self, node: *mut c_void) -> bool {
    let node = { node as *mut AstTypeTypeof };
    let expr = unsafe { (*node).expr };
    let name = mk_name_ast_expr(unsafe { &*expr });
    if let Some(name) = name {
      self.add(&name);
    }
    true
  }
}
