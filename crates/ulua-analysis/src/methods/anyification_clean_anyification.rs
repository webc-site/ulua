use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  records::{anyification::Anyification, table_type::TableType},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Anyification {
  /// 把已标脏的类型清洗为 Sealed/any 形态并返回新 arena 句柄。
  /// 对应 cpp `Analysis/src/Anyification.cpp:65`（`TypeId Anyification::clean(TypeId)`）。
  ///
  /// 降 safe 说明：`ty` 为 arena `TypeId` 句柄（同 `get_mutable_type_id` 门面纪律，
  /// 由 substitute 遍历传入）；`self.base.base.log` 是 Substitution 构造期
  /// `substitution_txn_log_type_arena` 注入并 LUAU_ASSERT 非空的会话期 TxnLog。
  /// 两处置裸引用都收进窄 `unsafe` 块，重建 `&TableType` 只读借用与 arena
  /// 追加写在本 pass 内单线程独占。
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(self.is_dirty_type_id(ty));

    let log = self.base.base.log;
    // Safety: log 为构造注入并断言非空的存活 *const TxnLog；ty 为遍历传入的
    // 存活 arena 节点句柄。txn_log_get_mutable 取 &self 只读遍历日志链定位可变槽位，
    // 按 T 类型转回 *mut TableType，命中返回对齐的非空指针、未命中返回 null，均不产生悬垂。
    let ttv = unsafe { (*log).txn_log_get_mutable::<TableType, TypeId>(ty) };
    if !ttv.is_null() {
      // Safety: 上一行已确认 ttv 非空，它是 txn_log_get_mutable 自存活 TxnLog/arena 定位的
      // *mut TableType（对齐由泛型转换保证），指向本 pass 独占、无其他活动可变借用的对象；单线程
      // 内重建 & 只读借用，仅在读字段构造 clone 期间存活，至 add_type 前不再经 log 改该表，无别名冲突。
      let ttv = unsafe { &*ttv };
      let mut clone = TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &ttv.props,
        ttv.indexer,
        ttv.level,
        TableState::Sealed,
      );
      clone.definition_module_name = ttv.definition_module_name.clone();
      clone.definition_location = ttv.definition_location;
      clone.name = ttv.name.clone();
      clone.synthetic_name = ttv.synthetic_name.clone();
      clone.tags = ttv.tags.clone();

      return self.base.add_type(clone);
    }

    self.any_type
  }

  /// 脏类型包一律折叠为 anyTypePack。对应 cpp `Analysis/src/Anyification.cpp:83`
  /// （`TypePackId Anyification::clean(TypePackId)`）。降 safe：`tp` 为 arena
  /// `TypePackId` 句柄，is_dirty 断言内部解引用已由其自身窄块收口，本函数无新解引用。
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    LUAU_ASSERT!(self.is_dirty_type_pack_id(tp));
    self.any_type_pack
  }
}
