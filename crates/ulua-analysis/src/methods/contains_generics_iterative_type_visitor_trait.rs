use crate::{
  records::{
    contains_generics::ContainsGenerics,
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl IterativeTypeVisitorTrait for ContainsGenerics {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    ContainsGenerics::visit_type_id(self, ty)
  }

  fn visit_type_id_generic_type(&mut self, ty: TypeId, gt: &GenericType) -> bool {
    ContainsGenerics::visit_type_id_generic_type(self, ty, gt)
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    ContainsGenerics::visit_type_id_type_function_instance_type(self, ty, tfit)
  }

  fn visit_type_pack_id_generic_type_pack(
    &mut self,
    tp: TypePackId,
    gtp: &GenericTypePack,
  ) -> bool {
    ContainsGenerics::visit_type_pack_id_generic_type_pack(self, tp, gtp)
  }
}
