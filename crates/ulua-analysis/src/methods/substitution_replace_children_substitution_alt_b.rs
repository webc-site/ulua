use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_mutable_type_pack::get_mutable_type_pack_id,
  records::{
    substitution::Substitution, type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_pack::TypePack, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl Substitution {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn replace_children_type_pack_id(&mut self, tp: TypePackId) {
    unsafe {
      LUAU_ASSERT!(tp == (*self.base.log).follow_type_pack_id(tp));
    }

    if self.base.ignore_children_type_pack_id(tp) {
      return;
    }

    if unsafe { (*tp).owning_arena != self.arena } {
      return;
    }

    if let Some(tpp) = get_mutable_type_pack_id::<TypePack>(tp) {
      for tv in tpp.head.iter_mut() {
        *tv = self.replace_type_id(*tv);
      }
      if let Some(tail) = tpp.tail {
        tpp.tail = Some(self.replace_type_pack_id(tail));
      }
    } else if let Some(vtp) = get_mutable_type_pack_id::<VariadicTypePack>(tp) {
      vtp.ty = self.replace_type_id(vtp.ty);
    } else if let Some(tfitp) = get_mutable_type_pack_id::<TypeFunctionInstanceTypePack>(tp) {
      for t in tfitp.type_arguments.iter_mut() {
        *t = self.replace_type_id(*t);
      }
      for t in tfitp.pack_arguments.iter_mut() {
        *t = self.replace_type_pack_id(*t);
      }
    }
  }
}
