use core::ffi::c_void;

use ulua_ast::records::ast_type_reference::AstTypeReference;

use crate::{
  functions::mk_name_topo_sort_statements_alt_d::mk_name_ast_name,
  records::arc_collector::ArcCollector,
};
impl ArcCollector {
  pub fn visit_ast_type_reference(&mut self, node: *mut c_void) -> bool {
    let node = { node as *mut AstTypeReference };
    let name = unsafe { (*node).name };
    let identifier = mk_name_ast_name(&name);
    self.add(&identifier);
    true
  }
}
