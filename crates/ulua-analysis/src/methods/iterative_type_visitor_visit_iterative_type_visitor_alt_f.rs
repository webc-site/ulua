use crate::{
  records::{iterative_type_visitor::IterativeTypeVisitor, primitive_type::PrimitiveType},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_primitive_type(&mut self, ty: TypeId, _ptv: &PrimitiveType) -> bool {
    self.visit_type_id(ty)
  }
}
