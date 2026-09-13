use core::ffi::CStr;

use ulua_ast::records::ast_name::AstName;

use crate::{
  functions::follow_type::follow_type_id,
  records::{
    infinite_type_finder::InfiniteTypeFinder, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::{name_type::Name, type_id::TypeId},
};
impl InfiniteTypeFinder {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    if self.found_infinite_type {
      return false;
    }

    let name = ast_name_to_name(petv.name);
    let tf = unsafe {
      let scope = self.scope.as_ref();
      if let Some(prefix) = petv.prefix {
        scope.lookup_imported_type(&ast_name_to_name(prefix), &name)
      } else {
        scope.lookup_type(&name)
      }
    };

    let Some(tf) = tf else {
      return true;
    };

    if follow_type_id(tf.r#type()) != follow_type_id(self.signature.fn_sig.r#type()) {
      return true;
    }

    for (argument, parameter) in petv.type_arguments.iter().zip(tf.type_params()) {
      if *argument != parameter.ty {
        self.found_infinite_type = true;
        return false;
      }
    }

    for (argument, parameter) in petv.pack_arguments.iter().zip(tf.type_pack_params()) {
      if *argument != parameter.tp {
        self.found_infinite_type = true;
        return false;
      }
    }

    false
  }
}

fn ast_name_to_name(name: AstName) -> Name {
  if name.value.is_null() {
    Name::new()
  } else {
    unsafe { CStr::from_ptr(name.value) }
      .to_string_lossy()
      .into_owned()
  }
}
