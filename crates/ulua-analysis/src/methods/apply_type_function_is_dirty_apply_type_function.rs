use crate::{
  functions::get_type,
  records::{apply_type_function::ApplyTypeFunction, free_type::FreeType},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyTypeFunction {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    if self.type_arguments.find(&ty).is_some() {
      true
    } else if let Some(ftv) = get_type::get::<FreeType>(ty) {
      if ftv.forwarded_type_alias {
        self.encountered_forwarded_type = true;
      }
      false
    } else {
      false
    }
  }

  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    self.type_pack_arguments.find(&tp).is_some()
  }
}
