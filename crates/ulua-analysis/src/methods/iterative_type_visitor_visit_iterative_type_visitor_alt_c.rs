use crate::{
  records::{free_type::FreeType, iterative_type_visitor::IterativeTypeVisitor},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, _ftv: &FreeType) -> bool {
    self.visit_type_id(ty)
  }
}
