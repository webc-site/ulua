use crate::{
  enums::table_state::TableState,
  functions::clone_clone::{pack_is_persistent, type_is_persistent},
  records::{
    anyification::Anyification, free_type::FreeType, free_type_pack::FreeTypePack,
    table_type::TableType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Anyification {
  /// 判定类型是否处于可被 anyify 的「脏」状态（Free/Unsealed table 或 FreeType）。
  /// 对应 C++ `bool Anyification::isDirty(TypeId ty)`（`cpp/Analysis/src/Anyification.cpp:41`）。
  ///
  /// 降 safe 说明：`ty` 是本 crate 通行的 arena `TypeId` 句柄（同 `get_type_id`
  /// 门面的既有纪律），两处置裸引用都收进下方窄 `unsafe` 块；
  /// `self.base.base.log` 由 [`Anyification`] 构造链经
  /// `Substitution::substitution_new(TxnLog::empty(), _)` 接线，恒非空。
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    // Safety: ty 为遍历中存活的 arena 类型句柄（TypeId 即 *const Type，arena
    // bump 分配地址不移动），persistent 为普通 bool 字段；log 字段构造期接
    // 线为 TxnLog::empty() 进程级单例（恒非空、比本对象长寿），(*log) 仅重建
    // &TxnLog 供只读查询；txn_log_get_mutable 按变体分派，非该变体返回 null，
    // ttv/ftv 均在 is_null 判定后才解引用，类型命中由 RTTI 式查询保证。
    unsafe {
      if type_is_persistent(ty) {
        return false;
      }

      let log = self.base.base.log;

      let ttv = (*log).txn_log_get_mutable::<TableType, TypeId>(ty);
      if !ttv.is_null() {
        return (*ttv).state == TableState::Free || (*ttv).state == TableState::Unsealed;
      }

      let ftv = (*log).txn_log_get_mutable::<FreeType, TypeId>(ty);
      !ftv.is_null()
    }
  }

  /// 判定类型包是否「脏」（FreeTypePack 即脏）。
  /// `bool Anyification::isDirty(TypePackId tp)` (Anyification.cpp:54-62)。
  ///
  /// 降 safe 说明：`tp` 为 arena `TypePackId` 句柄（同 `get_type_pack_id` 门面纪律），
  /// 解引用收进窄 `unsafe` 块；log 接线前提同 [`Anyification::is_dirty_type_id`]。
  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    // Safety: tp 为遍历中存活的 arena 类型包句柄（地址稳定），只读 persistent
    // 字段；log 构造期接线为非空 TxnLog::empty() 单例。
    if pack_is_persistent(tp) {
      return false;
    }

    // C++: `if (log->get_mutable<FreeTypePack>(tp)) return true; else return false;`
    let log = self.base.base.log;
    // Safety: log 构造期接线为非空 TxnLog::empty() 单例且长寿，(*log) 重建
    // &TxnLog 做只读查询；txn_log_get_mutable 未命中即返回 null，此处仅以
    // is_null 判空、不解引用 ftp，无别名与悬垂风险。
    let ftp = unsafe { (*log).txn_log_get_mutable::<FreeTypePack, TypePackId>(tp) };
    !ftp.is_null()
  }
}
