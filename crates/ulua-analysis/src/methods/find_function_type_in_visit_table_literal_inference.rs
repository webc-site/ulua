use crate::{records::find_function_type_in::FindFunctionTypeIn, type_aliases::type_id::TypeId};

impl FindFunctionTypeIn {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    false
  }
}
