use crate::{
  enums::follow_option::FollowOption, functions::follow_type_alt_c::follow_type_id_follow_option,
  records::type_cloner::TypeCloner, type_aliases::type_id::TypeId,
};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn find_type_id(&self, ty: TypeId) -> Option<TypeId> {
    let ty = unsafe { follow_type_id_follow_option(ty, FollowOption::DisableLazyTypeThunks) };

    if let Some(it) = unsafe { (*self.types).get(&ty) } {
      return Some(*it);
    } else if unsafe { (*ty).persistent } && ty != self.force_ty {
      return Some(ty);
    }

    None
  }
}
