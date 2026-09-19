use crate::{
  records::{
    find_function_type_in::FindFunctionTypeIn,
    function_type::FunctionType,
    intersection_type::IntersectionType,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitorTrait for FindFunctionTypeIn {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    FindFunctionTypeIn::visit_type_id(self, ty)
  }

  fn visit_type_id_union_type(&mut self, ty: TypeId, utv: &UnionType) -> bool {
    FindFunctionTypeIn::visit_type_id_union_type(self, ty, utv)
  }

  fn visit_type_id_intersection_type(&mut self, ty: TypeId, itv: &IntersectionType) -> bool {
    FindFunctionTypeIn::visit_type_id_intersection_type(self, ty, itv)
  }

  fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    FindFunctionTypeIn::visit_type_id_function_type(self, ty, ftv)
  }
}
