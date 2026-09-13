use crate::{
  records::{
    instantiation_queuer::InstantiationQueuer, reduce_constraint::ReduceConstraint,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId},
};

impl InstantiationQueuer {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    let solver = unsafe { &mut *self.solver };
    solver.push_constraint(
      self.scope,
      self.location,
      ConstraintV::Reduce(ReduceConstraint { ty }),
    );
    true
  }
}
