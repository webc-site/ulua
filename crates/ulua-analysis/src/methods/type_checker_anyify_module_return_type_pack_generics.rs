use alloc::vec::Vec;

use crate::{
  functions::{
    begin_type_pack::begin, end_type_pack::end_type_pack_id, follow_type, follow_type_pack,
    get_type, get_type_pack::get,
  },
  records::{
    generic_type::GenericType, type_checker::TypeChecker, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl TypeChecker {
  pub(crate) fn anyify_module_return_type_pack_generics(&mut self, tp: TypePackId) -> TypePackId {
    let tp = follow_type_pack::follow(tp);

    if let Some(vtp) = get::<VariadicTypePack>(tp) {
      let ty = follow_type::follow(vtp.ty);
      return if get_type::get::<GenericType>(ty).is_some() {
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

    let mut it = begin(tp);
    let end_it = end_type_pack_id(tp);

    while it != end_it {
      // SAFETY: iterator 解引用指向 arena 内类型。
      let ty = follow_type::follow(*it.current());
      result_types.push(if get_type::get::<GenericType>(ty).is_some() {
        self.any_type
      } else {
        ty
      });
      it.advance();
    }

    if let Some(tail) = it.tail() {
      result_tail = Some(self.anyify_module_return_type_pack_generics(tail));
    }

    self.add_type_pack_vector_type_id_optional_type_pack_id(&result_types, result_tail)
  }
}
