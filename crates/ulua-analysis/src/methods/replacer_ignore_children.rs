use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{extern_type::ExternType, function_type::FunctionType, replacer::Replacer},
  type_aliases::type_id::TypeId,
};

impl Replacer {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    unsafe {
      if !get_type_id::<ExternType>(ty).is_none() {
        return true;
      }

      if let Some(ftv) = get_type_id::<FunctionType>(ty).as_ref() {
        if ftv.has_no_free_or_generic_types {
          return false;
        }

        for &generic in &ftv.generics {
          if (*self.replacements).find(&generic).is_some() {
            return true;
          }
        }

        for &generic in &ftv.generic_packs {
          if (*self.replacement_packs).find(&generic).is_some() {
            return true;
          }
        }
      }
    }

    false
  }
}
