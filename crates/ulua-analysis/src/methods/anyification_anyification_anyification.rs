use crate::{
  macros::substitution_entry::substitution_entry,
  records::{
    anyification::Anyification, arena_handle::Handle, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
};

fn anyification_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 由 `install_substitution_vtable` 写入的 `self as *mut Anyification as
  // *mut ()`（见 tarjan::SubstitutionVtable 文档），只在该 Anyification 存活期内、
  // 单线程 substitute 遍历中作为回调派发，此刻无跨回调点的其他借用，故回转 *mut Anyification
  // 并重建 &mut 无别名冲突；入参 ty 为被替换强连通分量中 arena 存活的类型句柄，满足
  // is_dirty_type_id 读 `(*ty).persistent` 及 self.base.base.log 的契约。
  unsafe { (*(owner as *mut Anyification)).is_dirty_type_id(ty) }
}

fn anyification_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同 is_dirty_ty——owner 回转 *mut Anyification 的 &mut 仅在单线程 substitute 回调
  // 期内重建、无并存别名；入参 tp 为遍历中存活的 arena 类型包句柄，满足 is_dirty_type_pack_id
  // 读 `(*tp).persistent` 与 self.base.base.log 的契约。
  unsafe { (*(owner as *mut Anyification)).is_dirty_type_pack_id(tp) }
}

fn anyification_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: owner 回转 *mut Anyification 的 &mut 重建仅在单线程 substitute 回调期发生、无并存
  // 别名；入参 ty 为遍历中存活的 arena 类型句柄，clean_type_id 经 self.base（构造注入的
  // TxnLog::empty() 单例或活动 log）与 arena 读写替换结果，二者均比本次遍历长寿。
  unsafe { (*(owner as *mut Anyification)).clean_type_id(ty) }
}

fn anyification_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: 同 clean_ty——owner 回转 *mut Anyification 的 &mut 在单线程回调期内独占重建、
  // 无并存别名；tp 为遍历中存活的 arena 类型包句柄，clean_type_pack_id 只经构造期接线的
  // log/arena 读写，二者均比遍历长寿。
  unsafe { (*(owner as *mut Anyification)).clean_type_pack_id(tp) }
}

fn anyification_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: owner 回转 *mut Anyification 的 &mut 仅在单线程 substitute 回调期独占重建、无并存
  // 别名；`.base.found_dirty_type_id` 记录脏节点依赖，其入参 ty 为遍历中存活的 arena 类型句柄，
  // 且 Tarjan 内部经构造接线且比遍历长寿的 log 读写。
  unsafe { (*(owner as *mut Anyification)).base.found_dirty_type_id(ty) }
}

fn anyification_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  unsafe {
    // Safety: owner 回转 *mut Anyification 的 &mut 仅在单线程 substitute 回调期独占重建、无并存
    // 别名；`.base.found_dirty_type_pack_id` 记录脏依赖，入参 tp 为遍历中存活的 arena 类型包句柄，
    // Tarjan 内部只经构造接线且长寿的 log 读写。
    (*(owner as *mut Anyification))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn anyification_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 回转 *mut Anyification 的 &mut 仅在单线程 substitute 回调期独占重建、无并存
  // 别名；入参 ty 为遍历中存活的 arena 类型句柄，ignore_children_type_id 只读判定并经构造
  // 接线且长寿的 log 访问，满足其 unsafe 契约。
  unsafe { (*(owner as *mut Anyification)).ignore_children_type_id(ty) }
}

fn anyification_ignore_children_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同 ignore_children_ty——owner 回转 *mut Anyification 的 &mut 在单线程回调期内独占
  // 重建、无并存别名；tp 为遍历中存活的 arena 类型包句柄，ignore_children_type_pack_id 只经构造
  // 接线且长寿的 log 做只读判定。
  unsafe { (*(owner as *mut Anyification)).ignore_children_type_pack_id(tp) }
}

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

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Anyification as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(anyification_is_dirty_ty),
      is_dirty_tp: Some(anyification_is_dirty_tp),
      clean_ty: Some(anyification_clean_ty),
      clean_tp: Some(anyification_clean_tp),
      found_dirty_ty: Some(anyification_found_dirty_ty),
      found_dirty_tp: Some(anyification_found_dirty_tp),
      ignore_children_ty: Some(anyification_ignore_children_ty),
      ignore_children_tp: Some(anyification_ignore_children_tp),
      ignore_children_visit_ty: Some(anyification_ignore_children_ty),
      ignore_children_visit_tp: Some(anyification_ignore_children_tp),
    };
  }

  substitution_entry!(id, pack);

  /// 兼容旧 cpp 镜像签名（`Anyification(TypeArena*, NotNull<Scope>, BuiltinTypes*,
  /// InternalErrorReporter*, TypeId, TypePackId)`）：`scope`/`ice_handler` 在
  /// 本 Rust 端口中从未被读取（`base: Substitution` 自带 log/arena，任何化路径不
  /// 需要二者），故为已删除结构体字段的遗留入口，参数按 `_` 前缀忽略后转交
  /// [`Anyification::new`]。保留仅为不破坏 `ulua-unit-test` 等跨 crate 消费方。
  pub fn anyification_type_arena_scope_ptr_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
    arena: Handle<TypeArena>,
    _scope: &ScopePtr,
    builtin_types: Handle<BuiltinTypes>,
    _ice_handler: *mut InternalErrorReporter,
    any_type: TypeId,
    any_type_pack: TypePackId,
  ) -> Self {
    Self::new(arena, builtin_types, any_type, any_type_pack)
  }
}
