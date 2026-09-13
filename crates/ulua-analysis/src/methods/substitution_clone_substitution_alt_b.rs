use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{
    substitution::Substitution, type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_pack::TypePack, type_pack_var::TypePackVar, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl Substitution {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let mut tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    let ptp = unsafe { (*self.base.log).pending_type_pack_id(tp) };
    if !ptp.is_null() {
      tp = unsafe { &(*ptp).pending as *const TypePackVar };
    }

    if let Some(tpp) = get_type_pack_id::<TypePack>(tp) {
      let clone = TypePack {
        head: tpp.head.clone(),
        tail: tpp.tail,
      };
      return self.add_type_pack(clone);
    }

    if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp) {
      let clone = VariadicTypePack {
        ty: vtp.ty,
        hidden: vtp.hidden,
      };
      return self.add_type_pack(clone);
    }

    if let Some(tfitp) = get_type_pack_id::<TypeFunctionInstanceTypePack>(tp) {
      let clone = TypeFunctionInstanceTypePack {
        function: tfitp.function,
        type_arguments: tfitp.type_arguments.clone(),
        pack_arguments: tfitp.pack_arguments.clone(),
      };
      return self.add_type_pack(clone);
    }

    self.add_type_pack(unsafe { (*tp).clone() })
  }
}
