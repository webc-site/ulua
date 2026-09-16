use crate::{
  records::{intersection_type::IntersectionType, iterative_type_visitor::IterativeTypeVisitor},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_intersection_type(&mut self, ty: TypeId, _itv: &IntersectionType) -> bool {
    self.visit_type_id(ty)
  }
}
