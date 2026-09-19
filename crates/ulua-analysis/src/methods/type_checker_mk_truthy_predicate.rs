use alloc::boxed::Box;

use crate::{
  functions::{get_type_alt_j::get_type_id, is_nil::is_nil, is_prim::is_prim},
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton, free_type::FreeType,
    primitive_type::Type as PrimType, singleton_type::SingletonType, type_checker::TypeChecker,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId, type_id_predicate::TypeIdPredicate},
};
impl TypeChecker {
  /// `TypeIdPredicate TypeChecker::mkTruthyPredicate(bool sense, TypeId emptySetTy)`.
  /// Reference: `Analysis/src/TypeInfer.cpp:5544`.
  pub fn mk_truthy_predicate(&mut self, sense: bool, empty_set_ty: TypeId) -> TypeIdPredicate {
    // C++ captures `this` and calls `singletonType(sense)`; those are the builtin
    // true/false types, so we capture them directly to keep the Closure `Fn`.
    let true_type = unsafe { (*self.builtin_types).true_type };
    let false_type = unsafe { (*self.builtin_types).false_type };

    Box::new(move |ty: TypeId| -> Option<TypeId> {
      // any/error/free gets a special pass unconditionally because they can't be decided.
      if get_type_id::<AnyType>(ty).is_some()
        || get_type_id::<ErrorType>(ty).is_some()
        || get_type_id::<FreeType>(ty).is_some()
      {
        return Some(ty);
      }

      // maps boolean primitive to the corresponding singleton equal to sense
      if is_prim(ty, PrimType::Boolean) {
        return Some(if sense { true_type } else { false_type });
      }

      // if we have boolean singleton, eliminate it if the sense doesn't match with that singleton
      if let Some(stv) = get_type_id::<SingletonType>(ty)
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
    })
  }
}
