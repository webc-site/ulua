use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  functions::get_mutable_txn_log::{get_mutable_pending_type, get_mutable_pending_type_pack},
  records::{
    arena_handle::Handle, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, pending_type::PendingType, pending_type_pack::PendingTypePack,
    table_type::TableType, txn_log::TxnLog, type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  pub(crate) fn change_level_type_id_type_level(
    &mut self,
    ty: TypeId,
    new_level: TypeLevel,
  ) -> Handle<PendingType> {
    LUAU_ASSERT!(
      self.txn_log_is::<FreeType, TypeId>(ty)
        || self.txn_log_is::<TableType, TypeId>(ty)
        || self.txn_log_is::<FunctionType, TypeId>(ty)
    );

    // queue_type_id 要求 `ty` 指向存活 Type 节点——上方 LUAU_ASSERT 已确认其为
    // Free/Table/Function 之一的 arena 驻留节点；返回的 Handle 取自
    // type_var_changes 堆上 Box 的 `get_mut`，非空由类型编码、地址随 Box 稳定、
    // 由本 log 独占持有。
    let new_ty = self.queue_type_id(ty);
    // get_mutable_pending_type 以 class tag 向下分派，未命中返回 None（原 null
    // 哨兵），命中即 repr(C) 基址重合指向 pending 内嵌 Type 的对应变体的独占可变
    // 借用；写 `.level` 前均在 Some 分支内，PendingType 由本 `&mut self` log 独占，
    // 单线程语句级串行，无并存可变别名。
    if let Some(ftv) = get_mutable_pending_type::<FreeType>(new_ty) {
      ftv.level = new_level;
    } else if let Some(ttv) = get_mutable_pending_type::<TableType>(new_ty) {
      LUAU_ASSERT!(ttv.state == TableState::Free || ttv.state == TableState::Generic);
      ttv.level = new_level;
    } else if let Some(ftv) = get_mutable_pending_type::<FunctionType>(new_ty) {
      ftv.level = new_level;
    }

    new_ty
  }

  /// 调用序契约（正确性，非内存安全）：`tp` 须为本 log 所属 types arena 中
  /// `FreeTypePack` 节点的句柄（函数内 LUAU_ASSERT 以 safe 门面 `txn_log_is` 校验），
  /// 且本 `TxnLog` 处于独占可写状态；体内解引用全部经 safe 门面（`queue_type_pack_id`/
  /// `get_mutable_pending_type_pack`，pending 槽由本 log 堆上 Box 独占、地址稳定），
  /// 违约仅 LUAU_ASSERT 拦断或改错节点，不引入未定义行为。cpp `Analysis/src/TxnLog.cpp:430`。单线程串行。
  pub(crate) fn change_level_type_pack_id_type_level(
    &mut self,
    tp: TypePackId,
    new_level: TypeLevel,
  ) -> Handle<PendingTypePack> {
    LUAU_ASSERT!(self.txn_log_is::<FreeTypePack, TypePackId>(tp));

    // 同 TypeId 版——上方 LUAU_ASSERT 确认 `tp` 为 FreeTypePack 的存活 arena
    // 节点；queue_type_pack_id 返回 type_pack_changes 堆上 Box 的
    // `get_mut` 句柄，非空由 Handle 类型编码、地址稳定、由本 log 独占。
    let new_tp = self.queue_type_pack_id(tp);
    // class tag 命中才返回指向 pending 内嵌 FreeTypePack 的独占可变借用（未命中
    // 为 None，原 null 哨兵），`.level` 仅在 Some 分支写入，本 log 独占、
    // 单线程串行无别名。
    if let Some(ftp) = get_mutable_pending_type_pack::<FreeTypePack>(new_tp) {
      ftp.level = new_level;
    }

    new_tp
  }
}
