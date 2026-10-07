use crate::{
  records::{
    arena_handle::alias, clone_public_interface::ClonePublicInterface, type_error::TypeError,
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
      let module = alias(self.module);
      let location = module.scopes[0].0;
      module
        .errors
        .push(TypeError::type_error_location_type_error_data(
          location,
          UnificationTooComplex.into(),
        ));
      // Safety: self.builtin_types 为构造期接线的非空 BuiltinTypes 指针（指向进程/会话内置类型
      // 表），比 self 长寿；此处仅读取 error_type 字段（值拷贝），不产生并存别名借用。
      self.builtin_types.get_mut().error_type
    }
  }
}
