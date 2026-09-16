use crate::{
  records::{
    instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
    pending_expansion_type::PendingExpansionType,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId},
};

impl InstantiationQueuerDeprecated {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    let solver = unsafe { &mut *self.solver };
    solver.push_constraint(
      self.scope,
      self.location,
      ConstraintV::TypeAliasExpansion(TypeAliasExpansionConstraint { target: _ty }),
    );
    false
  }
}
