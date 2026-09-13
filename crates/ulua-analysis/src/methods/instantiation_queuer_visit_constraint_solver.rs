use crate::{
  records::{
    instantiation_queuer::InstantiationQueuer, pending_expansion_type::PendingExpansionType,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId},
};

impl InstantiationQueuer {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    let solver = unsafe { &mut *self.solver };
    solver.push_constraint(
      self.scope,
      self.location,
      ConstraintV::TypeAliasExpansion(TypeAliasExpansionConstraint { target: ty }),
    );
    false
  }
}
