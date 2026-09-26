use crate::{
  functions::get_type,
  records::{
    extern_type::ExternType, function_type::FunctionType, instantiation_2::Instantiation2,
  },
  type_aliases::type_id::TypeId,
};

impl Instantiation2 {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    if get_type::get::<ExternType>(ty).is_some() {
      return true;
    }

    if let Some(ftv) = get_type::get::<FunctionType>(ty).as_ref() {
      if ftv.has_no_free_or_generic_types {
        return false;
      }

      for &generic in &ftv.generics {
        if self.generic_substitutions.find(&generic).is_some() {
          return true;
        }
      }

      for &generic in &ftv.generic_packs {
        if self.generic_pack_substitutions.find(&generic).is_some() {
          return true;
        }
      }
    }

    false
  }
}
