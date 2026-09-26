use crate::{
  functions::{
    get_type,
    is_prim::{is_nil, is_prim},
  },
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton, free_type::FreeType,
    primitive_type::Type as PrimType, singleton_type::SingletonType, type_checker::TypeChecker,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId, type_id_predicate::TypeIdPredicate},
};
impl TypeChecker {
  /// `TypeIdPredicate TypeChecker::mkTruthyPredicate(bool sense, TypeId emptySetTy)`
  /// (`Analysis/src/TypeInfer.cpp:5534-5556`).
  ///
  /// 上游 lambda 捕获 `this` 只为调 `singletonType(sense)`（即 builtin 的
  /// true/false 类型），这里直接取那两个 Copy 值，闭包不借用 self；返回值用
  /// `use<>` 显式声明零生命周期捕获，调用点才能继续 `&mut self`。
  pub fn mk_truthy_predicate(
    &mut self,
    sense: bool,
    empty_set_ty: TypeId,
  ) -> impl TypeIdPredicate + use<> {
    // 契约：`self.builtin_types` 是 TypeChecker 构造期接线的 `Handle<BuiltinTypes>`
    // （记录体注释即 `// NotNull<BuiltinTypes>`），非空、对齐且指向比本 checker 长寿的
    // 会话单例。两处合并在同一作用域内、仅一次性拷贝 `true_type`/`false_type` 两个
    // `Copy` TypeId 句柄，因此不留 `&BuiltinTypes` 借用（闭包按 `use<>` 零捕获），
    // 后续调用方还能继续以 `&mut self` 驱动求解器而无双写别名。
    let builtin = self.builtin_types.get();
    let (true_type, false_type) = (builtin.true_type, builtin.false_type);

    move |_tc: &mut TypeChecker, ty: TypeId| -> Option<TypeId> {
      // any/error/free gets a special pass unconditionally because they can't be decided.
      if get_type::get::<AnyType>(ty).is_some()
        || get_type::get::<ErrorType>(ty).is_some()
        || get_type::get::<FreeType>(ty).is_some()
      {
        return Some(ty);
      }

      // maps boolean primitive to the corresponding singleton equal to sense
      if is_prim(ty, PrimType::Boolean) {
        return Some(if sense { true_type } else { false_type });
      }

      // if we have boolean singleton, eliminate it if the sense doesn't match with that singleton
      if let Some(stv) = get_type::get::<SingletonType>(ty)
        && let Some(boolean) = stv.variant.get_if::<BooleanSingleton>()
      {
        return if boolean.value == sense {
          Some(ty)
        } else {
          None
        };
      }

      // if we have nil, eliminate it if sense is true, otherwise take it
      if is_nil(ty) {
        return if sense { None } else { Some(ty) };
      }

      // at this point, anything else is kept if sense is true, or replaced by emptySetTy
      if sense { Some(ty) } else { Some(empty_set_ty) }
    }
  }
}
