use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::tracing_visitor::TracingVisitor;

impl TracingVisitor {
  pub fn cycle_type_id(&mut self, ty: TypeId) {
    self.cycles.push(ty);
  }
}
