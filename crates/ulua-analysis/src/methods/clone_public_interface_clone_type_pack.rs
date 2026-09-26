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
      // Safety: self.module 由构造期从调用方（cloneFromSourceModule 路径）的
      // 独占 &mut Module 接线为裸指针（C++ `Module& module` 引用成员的等价），
      // clone 遍历运行期内调用方暂停对该 Module 的一切其它借用，重建 &mut 无
      // 并存别名；作用域内仅 push errors 与只读 scopes[0]，且 Module 活过本
      // visitor。单线程串行，无数据竞争。
      let module_ref = unsafe { &mut *self.module };
      let location = module_ref.scopes[0].0;
      module_ref
        .errors
        .push(TypeError::type_error_location_type_error_data(
          location,
          UnificationTooComplex::default().into(),
        ));
      // Safety: self.builtin_types 构造期接线的 *mut BuiltinTypes 指向进程级
      // 长寿的 builtin 单例（C++ `BuiltinTypes*` 成员，恒非空、比持有者长寿），
      // 此处只读其 error_type_pack 句柄字段，瞬态借用止于本表达式。
      self.builtin_types.get_mut().error_type_pack
    }
  }
}
