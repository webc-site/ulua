use crate::{
  enums::table_state::TableState,
  records::{
    generic_type::GenericType, generic_type_pack::GenericTypePack,
    replace_generics::ReplaceGenerics, table_type::TableType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ReplaceGenerics {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let log = self.base.base.log;

    // Safety: self.base.base.log 是 Substitution/Tarjan 构造注入的非空 *const TxnLog（empty 单例或
    // 会话活动 log），比 ReplaceGenerics 长寿；txn_log_get_mutable 取 &self 只读 log，返回 log/arena
    // 中 TableType 可变视图裸指针（可能为 null，下一行判空）；ty 为存活 arena 句柄。
    let ttv = unsafe { (*log).txn_log_get_mutable::<TableType, TypeId>(ty) };
    if !ttv.is_null() {
      // Safety: 上一行 `!ttv.is_null()` 已判空——txn_log_get_mutable 依 RTTI 命中 TableType 才返回
      // 非空，故 ttv 指向本次 log 视图内存活且类型正确的节点，只读其 state。
      return unsafe { (*ttv).state } == TableState::Generic;
    }

    // Safety: 同上 log 非空长寿；取 GenericType 可变视图裸指针（可能为 null），ty 为存活 arena 句柄。
    let gtv = unsafe { (*log).txn_log_get_mutable::<GenericType, TypeId>(ty) };
    !gtv.is_null() && self.generics.contains(&ty)
  }

  pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
    let log = self.base.base.log;
    // 对齐 cpp Instantiation.cpp:141-147：仅当 log 的 pending 态命中 GenericTypePack 时
    // 才做 genericPacks 成员判断，否则直接视为 not-dirty。
    // Safety: self.base.base.log 为 Substitution/Tarjan 构造注入的非空 TxnLog（empty 单例或活动
    // log），比 ReplaceGenerics 长寿；txn_log_get_mutable 取 &self 只读，返回 GenericTypePack 可变
    // 视图裸指针（可能为 null），tp 为存活 arena 类型包句柄。
    let gtp = unsafe { (*log).txn_log_get_mutable::<GenericTypePack, TypePackId>(tp) };
    !gtp.is_null() && self.generic_packs.contains(&tp)
  }
}
