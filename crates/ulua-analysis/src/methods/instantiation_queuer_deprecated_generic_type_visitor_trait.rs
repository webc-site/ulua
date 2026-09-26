use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    extern_type::ExternType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
    pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
    visit_key::VisitKey,
  },
  type_aliases::type_id::TypeId,
};

impl GenericTypeVisitorTrait for InstantiationQueuerDeprecated {
  type Seen = DenseHashSet<VisitKey>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    InstantiationQueuerDeprecated::visit_type_id_pending_expansion_type(self, ty, petv)
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    InstantiationQueuerDeprecated::visit_type_id_type_function_instance_type(self, ty, tfit)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    InstantiationQueuerDeprecated::visit_type_id_extern_type(self, ty, etv)
  }
}
