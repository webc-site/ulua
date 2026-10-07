//! `anyification` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  functions::{
    clone_clone::{pack_is_persistent, type_is_persistent},
    get_type,
  },
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    anyification::Anyification,
    arena_handle::{Handle, alias_nn_opt, alias_ref},
    builtin_types::BuiltinTypes,
    extern_type::ExternType,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    substitution::Substitution,
    table_type::TableType,
    txn_log::TxnLog,
    type_arena::TypeArena,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
};

// TypePackId 侧三覆写槽全为真实转发（见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(real, Anyification, ic = ignore_children_type_id);
impl Anyification {
  pub fn new(
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    any_type: TypeId,
    any_type_pack: TypePackId,
  ) -> Self {
    Anyification {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      builtin_types,
      any_type,
      any_type_pack,
      normalization_too_complex: false,
    }
  }

  substitution_entry!(id, pack);

  /// 兼容旧 cpp 镜像签名（`Anyification(TypeArena*, NotNull<Scope>, BuiltinTypes*,
  /// InternalErrorReporter*, TypeId, TypePackId)`）：`scope`/`ice` 在本 Rust 端口中
  /// 从未被读取（`base: Substitution` 自带 log/arena，任何化路径不需要二者）。
  /// `ice` 死形参连同唯一调用点的实参一并删除（review.md §3 死代码规则），`_scope`
  /// 仍为已删除结构体字段的遗留入口，按 `_` 前缀忽略后转交 [`Anyification::new`]。
  /// 保留仅为不破坏 `ulua-unit-test` 等跨 crate 消费方。
  pub fn anyification_type_arena_scope_ptr_not_null_builtin_types_type_id_type_pack_id(
    arena: Handle<TypeArena>,
    _scope: &ScopePtr,
    builtin_types: Handle<BuiltinTypes>,
    any_type: TypeId,
    any_type_pack: TypePackId,
  ) -> Self {
    Self::new(arena, builtin_types, any_type, any_type_pack)
  }
}

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
    let ttv = alias_ref(log).txn_log_get_mutable::<TableType, TypeId>(ty);
    if let Some(ttv) = alias_nn_opt(ttv) {
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

impl Anyification {
  /// extern 类型不参与替换遍历。对应 C++ `bool Anyification::ignoreChildren(TypeId ty)`
  /// （Anyification.cpp:89-94）。降 safe：`ty` 为 arena 句柄（同 `get_type_id` 门面
  /// 纪律），`(*ty).persistent` 解引用收进窄 `unsafe` 块。
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let et = get_type::get::<ExternType>(ty);
    if et.is_some() {
      return true;
    }

    // type_is_persistent（clone_clone 的 pub(crate) 探针）内部窄块证成句柄前提。
    type_is_persistent(ty)
  }

  /// 类型包孪生版；降 safe 理由同上。
  /// `bool Anyification::ignoreChildren(TypePackId ty)` (Anyification.cpp:96-101).
  pub fn ignore_children_type_pack_id(&mut self, ty: TypePackId) -> bool {
    // 同 TypeId 版：pack_is_persistent 探针内已证成 arena 句柄前提。
    pack_is_persistent(ty)
  }
}

impl Anyification {
  /// 判定类型是否处于可被 anyify 的「脏」状态（Free/Unsealed table 或 FreeType）。
  /// 对应 C++ `bool Anyification::isDirty(TypeId ty)`（`cpp/Analysis/src/Anyification.cpp:41`）。
  ///
  /// 降 safe 说明：`ty` 是本 crate 通行的 arena `TypeId` 句柄（同 `get_type_id`
  /// 门面的既有纪律），两处置裸引用都收进下方窄 `unsafe` 块；
  /// `self.base.base.log` 由 [`Anyification`] 构造链经
  /// `Substitution::substitution_new(TxnLog::empty(), _)` 接线，恒非空。
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    if type_is_persistent(ty) {
      return false;
    }

    let log = self.base.base.log;

    let ttv = alias_ref(log).txn_log_get_mutable::<TableType, TypeId>(ty);
    if let Some(ttv) = alias_nn_opt(ttv) {
      return ttv.state == TableState::Free || ttv.state == TableState::Unsealed;
    }

    let ftv = alias_ref(log).txn_log_get_mutable::<FreeType, TypeId>(ty);
    ftv.is_some()
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
    let ftp = alias_ref(log).txn_log_get_mutable::<FreeTypePack, TypePackId>(tp);
    ftp.is_some()
  }
}
