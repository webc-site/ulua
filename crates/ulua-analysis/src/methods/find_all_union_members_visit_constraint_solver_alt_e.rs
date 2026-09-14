use crate::{
  records::{
    find_all_union_members::FindAllUnionMembers,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

impl FindAllUnionMembers {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    _ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    self.blocked_tys.insert_type_id(_ty);
    false
  }
}
