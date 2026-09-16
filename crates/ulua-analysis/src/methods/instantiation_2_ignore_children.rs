use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    extern_type::ExternType, function_type::FunctionType, instantiation_2::Instantiation2,
  },
  type_aliases::type_id::TypeId,
};

impl Instantiation2 {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    if !get_type_id::<ExternType>(ty).is_none() {
      return true;
    }

    if let Some(ftv) = get_type_id::<FunctionType>(ty).as_ref() {
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
