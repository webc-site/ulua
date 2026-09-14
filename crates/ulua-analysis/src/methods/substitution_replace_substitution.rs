use crate::{records::substitution::Substitution, type_aliases::type_id::TypeId};

impl Substitution {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn replace_type_id(&mut self, ty: TypeId) -> TypeId {
    let ty = unsafe { (*self.base.log).follow_type_id(ty) };
    match self.new_types.find(&ty) {
      Some(prev_ty) => *prev_ty,
      None => ty,
    }
  }
}
