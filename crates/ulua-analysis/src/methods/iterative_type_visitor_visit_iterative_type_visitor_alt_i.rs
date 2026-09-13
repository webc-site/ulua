use crate::{
  records::{iterative_type_visitor::IterativeTypeVisitor, metatable_type::MetatableType},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_metatable_type(&mut self, ty: TypeId, _mtv: &MetatableType) -> bool {
    self.visit_type_id(ty)
  }
}
