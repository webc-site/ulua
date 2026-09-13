use core::ffi::c_void;

use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

use crate::{
  functions::mk_name_topo_sort_statements_alt_l::mk_name_ast_stat_type_alias,
  records::arc_collector::ArcCollector,
};
impl ArcCollector {
  pub fn visit_ast_stat_type_alias(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatTypeAlias;
    let name = mk_name_ast_stat_type_alias(unsafe { &*node });
    self.add(&name);
    true
  }
}
