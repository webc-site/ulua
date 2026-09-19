use crate::{
  records::{
    clone_public_interface::ClonePublicInterface, type_error::TypeError,
    unification_too_complex::UnificationTooComplex,
  },
  type_aliases::type_id::TypeId,
};

impl ClonePublicInterface {
  /// `TypeId ClonePublicInterface::cloneType(TypeId ty)`.
  /// Reference: `Module.cpp:231-245`.
  pub fn clone_type(&mut self, ty: TypeId) -> TypeId {
    // C++: std::optional<TypeId> result = substitute(ty); (inherited from Substitution)
    self.install_substitution_vtable();
    let result = self.base.substitute_type_id(ty);
    if let Some(r) = result {
      r
    } else {
      // C++: module->errors.emplace_back(module->scopes[0].first, UnificationTooComplex{});
      //      return builtinTypes->error_type;
      let module = unsafe { &mut *self.module };
      let location = module.scopes[0].0;
      module
        .errors
        .push(TypeError::type_error_location_type_error_data(
          location,
          UnificationTooComplex::default().into(),
        ));
      unsafe { (*self.builtin_types).error_type }
    }
  }
}
