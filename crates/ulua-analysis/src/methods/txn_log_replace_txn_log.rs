use crate::{
  records::{
    pending_type::PendingType, pending_type_pack::PendingTypePack, txn_log::TxnLog, r#type::Type,
    type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  pub fn replace_type_id_t<T>(&mut self, ty: TypeId, replacement: T) -> *mut PendingType
  where
    T: Into<Type>,
  {
    let replacement_type: Type = replacement.into();
    self.replace_type_id_type_item(ty, replacement_type)
  }

  pub(crate) fn replace_type_id_type_item(
    &mut self,
    _ty: TypeId,
    _replacement: Type,
  ) -> *mut PendingType {
    // Safety: queue_type_id 为 unsafe fn，其「ty 指向存活 Type 节点」前置由调用
    // 方契约满足（C++ ReplaceType 以 arena/pending 存活句柄入参）；返回指针源自
    // self.type_var_changes 中 Box<PendingType> 表项，Box 堆址稳定且只要 log 不
    // 被回滚清空即与 self 同寿——本函数持有 &mut self，表项在两次使用间必然存活。
    let new_ty = unsafe { self.queue_type_id(_ty) };
    // Safety: new_ty 如上非空存活；reassign 经 (*new_ty).pending 取 &mut，此刻
    // 该表项仅此一处可变访问（&mut self 排他了路径），单线程串行无并存别名。
    unsafe {
      (*new_ty).pending.reassign(&_replacement);
    }
    new_ty
  }

  pub(crate) fn replace_type_pack_id_type_pack_var(
    &mut self,
    tp: TypePackId,
    replacement: TypePackVar,
  ) -> *mut PendingTypePack {
    // Safety: 与 TypeId 侧对称——queue_type_pack_id 要求 tp 存活（调用方按 C++
    // ReplaceTypePack 契约传入 arena/pending 句柄）；返回值源自 type_pack_changes
    // 的 Box<PendingTypePack> 表项，堆址稳定且与本 &mut self 同寿。
    let new_tp = unsafe { self.queue_type_pack_id(tp) };
    // Safety: new_tp 非空存活（同上）；对表项 pending 的 &mut 写入是本函数内
    // 对该对象的唯一访问，串行窗口无别名冲突。
    unsafe {
      (*new_tp).pending.reassign(&replacement);
    }
    new_tp
  }
}
