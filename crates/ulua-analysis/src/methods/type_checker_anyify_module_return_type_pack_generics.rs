use alloc::vec::Vec;

use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end_type_pack_id,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get,
  },
  records::{
    generic_type::GenericType, type_checker::TypeChecker, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl TypeChecker {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn anyify_module_return_type_pack_generics(&mut self, tp: TypePackId) -> TypePackId {
    let tp = unsafe { follow_type_pack_id(tp) };

    if let Some(vtp) = get::<VariadicTypePack>(tp) {
      let ty = follow_type_id(vtp.ty);
      return if get_type_id::<GenericType>(ty).is_some() {
        self.any_type_pack
      } else {
        tp
      };
    }

    if get::<TypePack>(tp).is_none() {
      return tp;
    }

    let mut result_types = Vec::new();
    let mut result_tail = None;

    let mut it = begin_type_pack_id(tp);
    let end_it = end_type_pack_id(tp);

    while it.operator_ne(&end_it) {
      // SAFETY: iterator 解引用指向 arena 内类型。
      let ty = follow_type_id(*it.operator_deref());
      result_types.push(if get_type_id::<GenericType>(ty).is_some() {
        self.any_type
      } else {
        ty
      });
      it.operator_inc();
    }

    if let Some(tail) = it.tail() {
      result_tail = Some(self.anyify_module_return_type_pack_generics(tail));
    }

    self.add_type_pack_vector_type_id_optional_type_pack_id(&result_types, result_tail)
  }
}
