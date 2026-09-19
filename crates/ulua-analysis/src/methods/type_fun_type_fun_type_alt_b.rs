use crate::{records::type_fun::TypeFun, type_aliases::type_id::TypeId};

impl TypeFun {
  pub fn type_fun_type_id(r#type: TypeId) -> Self {
    Self {
      type_params: Vec::new(),
      type_pack_params: Vec::new(),
      r#type,
      definition_location: None,
    }
  }
}
