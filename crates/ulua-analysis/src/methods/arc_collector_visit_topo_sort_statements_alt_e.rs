use core::ffi::c_void;

use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

use crate::{
  functions::mk_name_topo_sort_statements_alt_i::mk_name_ast_stat_local_function,
  records::arc_collector::ArcCollector,
};
impl ArcCollector {
  pub fn visit_ast_stat_local_function(&mut self, node: *mut c_void) -> bool {
    let node = unsafe { &*node.cast::<AstStatLocalFunction>() };
    let name = mk_name_ast_stat_local_function(node);
    self.add(&name);
    true
  }
}
