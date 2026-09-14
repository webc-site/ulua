use core::ffi::c_void;

use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::records::require_tracer::RequireTracer;
impl RequireTracer<'_> {
  pub fn visit_ast_stat_local(&mut self, node: *mut c_void) -> bool {
    let stat = node as *mut AstStatLocal;
    let stat_ref = unsafe { &*stat };

    // zip 在较短一侧停摆，等价于上游的 min(limit) 逐对读取
    for (&local, &expr) in stat_ref
      .vars
      .as_slice()
      .iter()
      .zip(stat_ref.values.as_slice())
    {
      self.locals.try_insert(local, expr);
    }

    true
  }
}
