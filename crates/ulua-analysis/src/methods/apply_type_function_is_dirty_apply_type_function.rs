use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{apply_type_function::ApplyTypeFunction, free_type::FreeType},
  type_aliases::type_id::TypeId,
};

impl ApplyTypeFunction {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    if self.type_arguments.find(&ty).is_some() {
      true
    } else if let Some(ftv) = get_type_id::<FreeType>(ty) {
      if ftv.forwarded_type_alias {
        self.encountered_forwarded_type = true;
      }
      false
    } else {
      false
    }
  }
}
