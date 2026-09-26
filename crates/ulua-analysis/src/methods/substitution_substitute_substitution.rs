use crate::{
  enums::tarjan_result::TarjanResult,
  records::substitution::Substitution,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Substitution {
  pub(crate) fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
    // Safety: `self.base.log` 对应 C++ `NotNull<TxnLog>`，构造时以进程级单例
    // `TxnLog::empty()` 接线、恒非空且永生；`follow_type_id` 只读该事务日志。
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
          // replace_children_type_id 已降 safe：log/arena 句柄前提在其窄块内证成。
          self.replace_children_type_id(*new_ty);
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
          // 同上：replace_children_type_pack_id 已降 safe。
          self.replace_children_type_pack_id(*new_tp);
        }
        self.replaced_type_packs.insert(*new_tp);
      }
    }

    let new_ty = self.replace_type_id(ty);
    Some(new_ty)
  }

  pub(crate) fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
    // Safety: `self.base.log` 为进程级单例 `TxnLog::empty()` 接线的非空、永生指针；
    // `follow_type_pack_id` 只读该事务日志。
    let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    self.base.clear_tarjan(self.base.log);

    let result = self.base.find_dirty_type_pack_id(tp);
    if result != TarjanResult::Ok {
      return None;
    }

    let new_types_clone = self.new_types.clone();
    for (old_ty, new_ty) in new_types_clone.iter() {
      if !self.base.ignore_children_type_id(*old_ty) && !self.replaced_types.contains(new_ty) {
        if !self.no_traverse_types.contains(new_ty) {
          // replace_children_type_id 已降 safe。
          self.replace_children_type_id(*new_ty);
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
          // 同上：replace_children_type_pack_id 已降 safe。
          self.replace_children_type_pack_id(*new_tp);
        }
        self.replaced_type_packs.insert(*new_tp);
      }
    }

    Some(self.replace_type_pack_id(tp))
  }
}
