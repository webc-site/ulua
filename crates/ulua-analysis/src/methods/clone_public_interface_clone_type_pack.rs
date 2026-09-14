use crate::{
  records::{
    clone_public_interface::ClonePublicInterface, type_error::TypeError,
    unification_too_complex::UnificationTooComplex,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl ClonePublicInterface {
  /// `TypePackId ClonePublicInterface::cloneTypePack(TypePackId tp)`.
  /// Reference: `Module.cpp:246-259`.
  pub fn clone_type_pack(&mut self, tp: TypePackId) -> TypePackId {
    // C++: std::optional<TypePackId> result = substitute(tp); (inherited from Substitution)
    self.install_substitution_vtable();
    let result = self.base.substitute_type_pack_id(tp);
    if let Some(res) = result {
      res
    } else {
      // C++: module->errors.emplace_back(module->scopes[0].first, UnificationTooComplex{});
      //      return builtinTypes->error_type_pack;
      let module_ref = unsafe { &mut *self.module };
      let location = module_ref.scopes[0].0;
      module_ref
        .errors
        .push(TypeError::type_error_location_type_error_data(
          location,
          UnificationTooComplex::default().into(),
        ));
      unsafe { (*self.builtin_types).error_type_pack }
    }
  }
}
