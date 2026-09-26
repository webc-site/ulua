//! Source: `Analysis/src/Unifier.cpp:23-141` (hand-ported)
//!
//! C++ `struct PromoteTypeLevels final : TypeOnceVisitor`. The visitor itself
//! does not customize traversal (unlike `FreeTypeSearcher`), so we wire it to
//! the base `GenericTypeVisitor::traverse` by implementing
//! `GenericTypeVisitorTrait`. This is what makes `promoteTypeLevels` actually
//! recurse and fire `log.changeLevel(...)`; the entry points call
//! `traverse_type_id` / `traverse_type_pack_id` rather than a single `visit`.
//!
//! The per-node `visit(...)` overrides (Unifier.cpp:49-120) are inlined here so
//! the live traversal path is self-contained and faithful to the C++ guards
//! (arena ownership, `log.is<T>` "uncommitted bound" check, table `Free`/
//! `Generic` state filter).

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_type::FunctionType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    promote_type_levels::PromoteTypeLevels,
    table_type::TableType,
    r#type::Type,
    type_pack_var::TypePackVar,
  },
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl GenericTypeVisitorTrait for PromoteTypeLevels {
  type Seen = DenseHashSet<*mut ()>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  /// `bool visit(TypeId ty) override` (Unifier.cpp:49-56).
  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    // Safety: TypeId 约定为非空对齐的 arena 节点句柄，且经 TypeOnceVisitor 遍历栈
    // 传入（节点在本遍历期存活）；此处只读头部 owning_arena 身份字段与本 pass 的
    // self.type_arena_id 值比较，不触碰 ty 的载荷。
    unsafe {
      // 中转 cast 与 `visit_type_pack_id` 一致：先转局部裸指针再解引用
      let ty_ptr: *const Type = ty;
      // Type levels of types from other modules are already global.
      if (*ty_ptr).owning_arena != self.type_arena_id {
        return false;
      }
    }
    true
  }

  /// `bool visit(TypePackId tp) override` (Unifier.cpp:58-65).
  fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    // Safety: TypePackId 同为非空对齐、遍历期存活的 arena pack 句柄；只读其
    // owning_arena 身份字段做归属比较，无写入。
    unsafe {
      let tp_var: *const TypePackVar = tp;
      if (*tp_var).owning_arena != self.type_arena_id {
        return false;
      }
    }
    true
  }

  /// `bool visit(TypeId ty, const FreeType&) override` (Unifier.cpp:67-76).
  fn visit_type_id_free_type(&mut self, ty: TypeId, _ftv: &FreeType) -> bool {
    // Safety: self.log 由构造函数从 `&mut TxnLog` 接线，指向 unifier 会话 log，
    // 在本 visitor 遍历期间存活且未被移动；is/get_mutable 是 log 自有 map 的查询，
    // BoundType 短路保证后续 get_mutable 只在节点仍为 FreeType 时经 log 返回稳定
    // 载荷指针（txn log pending 记录 Box 地址不移动），(*ft).level 读取随之有效。
    unsafe {
      // Surprise, it's actually a BoundType that hasn't been committed
      // yet. Calling get_mutable on this will trigger an assertion — and so
      // would `is::<FreeType>` below, because it goes *through* get_mutable.
      // `is::<BoundType>` is the one query get_mutable permits without
      // asserting, so use it to detect (and skip) the now-bound case.
      if (*self.log).txn_log_is::<BoundType, TypeId>(ty) {
        return true;
      }
      if !(*self.log).txn_log_is::<FreeType, TypeId>(ty) {
        return true;
      }
      let ft = (*self.log).txn_log_get_mutable::<FreeType, TypeId>(ty);
      self.promote(ty, (*ft).level);
    }
    true
  }

  /// `bool visit(TypeId ty, const FunctionType&) override` (Unifier.cpp:78-91).
  fn visit_type_id_function_type(&mut self, ty: TypeId, _ftv: &FunctionType) -> bool {
    // Safety: ty 为遍历栈携带的 arena 存活节点句柄（TypeId 非空对齐约定）；self.log
    // 构造期从 `&mut TxnLog` 接线、遍历期存活。BoundType 短路后 is/get_mutable
    // 只在节点仍为 FunctionType 时经 log 取稳定
    // pending 载荷指针，(*ft).level 读取有效（同 visit_type_id_free_type 的 log 不变量）。
    unsafe {
      // 中转 cast 与 `visit_type_id` 一致：先转局部裸指针再解引用
      let ty_ptr: *const Type = ty;
      if (*ty_ptr).owning_arena != self.type_arena_id {
        return false;
      }
      // Mirror `visit_type_id_free_type`: the txn log may have bound this
      // type without committing; `is::<FunctionType>` goes through get_mutable
      // and asserts on a bound type, so short-circuit on `is::<BoundType>`.
      if (*self.log).txn_log_is::<BoundType, TypeId>(ty) {
        return true;
      }
      if !(*self.log).txn_log_is::<FunctionType, TypeId>(ty) {
        return true;
      }
      let ft = (*self.log).txn_log_get_mutable::<FunctionType, TypeId>(ty);
      self.promote(ty, (*ft).level);
    }
    true
  }

  /// `bool visit(TypeId ty, const TableType& ttv) override` (Unifier.cpp:93-109).
  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    use crate::enums::table_state::TableState;
    // Safety: 同 free/function 分支——ty 是遍历期存活的 arena 节点句柄，self.log
    // 指向 unifier 会话 TxnLog（构造期接线、遍历期存活）；ttv 由遍历器从同一节点
    // 借出只读快照；BoundType 短路保证 txn_log_get_mutable 仅在节点仍为 TableType
    // 时返回 log 自有 pending 记录的稳定指针，(*ttv_mut).level 读取有效。
    unsafe {
      // 中转 cast 与 `visit_type_id` 一致：先转局部裸指针再解引用
      let ty_ptr: *const Type = ty;
      if (*ty_ptr).owning_arena != self.type_arena_id {
        return false;
      }

      if ttv.state != TableState::Free && ttv.state != TableState::Generic {
        return true;
      }

      // Mirror `visit_type_id_free_type`: the txn log may have bound this
      // type without committing; `is::<TableType>` goes through get_mutable
      // and asserts on a bound type, so short-circuit on `is::<BoundType>`.
      if (*self.log).txn_log_is::<BoundType, TypeId>(ty) {
        return true;
      }
      if !(*self.log).txn_log_is::<TableType, TypeId>(ty) {
        return true;
      }
      let ttv_mut = (*self.log).txn_log_get_mutable::<TableType, TypeId>(ty);
      self.promote(ty, (*ttv_mut).level);
    }
    true
  }

  /// `bool visit(TypePackId tp, const FreeTypePack&) override` (Unifier.cpp:111-120).
  fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, _ftp: &FreeTypePack) -> bool {
    // Safety: tp 为遍历栈携带的非空对齐 arena pack 句柄；self.log 构造期接线、
    // 遍历期存活。BoundTypePack 短路后 txn_log_get_mutable 只在 log 自有 pending
    // 记录（Box 稳定地址）中返回载荷指针，(*ftp).level 读取与 TypeId 侧同理。
    unsafe {
      // Mirror the TypeId path (`visit_type_id_free_type`): the pack may
      // actually be a BoundTypePack that the txn log hasn't committed yet.
      // `get_mutable`/`is::<FreeTypePack>` both go *through* get_mutable and
      // assert on a bound pack; `is::<BoundTypePack>` is the one query
      // get_mutable permits, so use it to detect and skip the now-bound case.
      if (*self.log).txn_log_is::<BoundTypePack, TypePackId>(tp) {
        return true;
      }
      if !(*self.log).txn_log_is::<FreeTypePack, TypePackId>(tp) {
        return true;
      }
      let ftp = (*self.log).txn_log_get_mutable::<FreeTypePack, TypePackId>(tp);
      self.promote_pack(tp, (*ftp).level);
    }
    true
  }
}
