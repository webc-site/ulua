use core::ffi::c_void;

use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::records::arc_collector::ArcCollector;
impl ArcCollector {
  pub fn visit_ast_type_pack(&mut self, node: *mut c_void) -> bool {
    let typed_node = node as *mut AstTypePack;
    let _ = typed_node;
    true
  }
}
