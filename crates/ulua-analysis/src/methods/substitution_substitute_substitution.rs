use crate::{
  enums::tarjan_result::TarjanResult, records::substitution::Substitution,
  type_aliases::type_id::TypeId,
};

impl Substitution {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
    let ty = unsafe { (*self.base.log).follow_type_id(ty) };

    self.base.clear_tarjan(self.base.log);

    let result = self.base.find_dirty_type_id(ty);
    if result != TarjanResult::Ok {
      return None;
    }

    let new_types_clone = self.new_types.clone();
    for (old_ty, new_ty) in new_types_clone.iter() {
      if !self.base.ignore_children_type_id(*old_ty) && !self.replaced_types.contains(new_ty) {
        if !self.no_traverse_types.contains(new_ty) {
          unsafe { self.replace_children_type_id(*new_ty) };
        }
        self.replaced_types.insert(*new_ty);
      }
    }

    let new_packs_clone = self.new_packs.clone();
    for (old_tp, new_tp) in new_packs_clone.iter() {
      if !self.base.ignore_children_type_pack_id(*old_tp)
        && !self.replaced_type_packs.contains(new_tp)
      {
        if !self.no_traverse_type_packs.contains(new_tp) {
          unsafe { self.replace_children_type_pack_id(*new_tp) };
        }
        self.replaced_type_packs.insert(*new_tp);
      }
    }

    let new_ty = self.replace_type_id(ty);
    Some(new_ty)
  }
}
