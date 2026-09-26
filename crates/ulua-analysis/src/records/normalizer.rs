use alloc::{collections::BTreeMap, sync::Arc};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, normalized_type::NormalizedType,
    type_arena::TypeArena, type_id_pair_hash::TypeIdPairHash, type_ids::TypeIds,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct Normalizer {
  pub(crate) cached_normals: BTreeMap<TypeId, Arc<NormalizedType>>,
  pub(crate) cached_intersections: BTreeMap<*const TypeIds, TypeId>,
  pub(crate) cached_unions: BTreeMap<*const TypeIds, TypeId>,
  pub(crate) cached_type_ids: BTreeMap<*const TypeIds, Box<TypeIds>>,
  pub(crate) cached_is_inhabited: DenseHashMap<TypeId, bool>,
  pub(crate) cached_is_inhabited_intersection: DenseHashMap<(TypeId, TypeId), bool, TypeIdPairHash>,
  pub(crate) fuel: Option<i32>,
  // `arena` 用 `Option<Handle>`：null 是有语义的哨兵（模块外归一化时经 ice_handler
  // 上报），见 `Normalizer::normalize_uncaught` 的 `is_none` 分支；TypeChecker 的
  // 归一化 arena 亦在每次 check 前才接线（两段式），构造期可为 None。
  pub(crate) arena: Option<Handle<TypeArena>>,
  // 原 `*mut BuiltinTypes`（C++ NotNull）收敛为句柄：指向宿主常驻单例，恒非空。
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  // C++ 形参是 `NotNull<UnifierSharedState>`，但 `TypeChecker` 的成员互引只能在
  // 对象于 Box 中定址后接线（两段式），故构造期可为 None，使用前必经
  // `shared_state_ref/mut` 断言接线（`shared_state_ptr` 保留 null 透传语义）。
  pub(crate) shared_state: Option<Handle<UnifierSharedState>>,
  pub(crate) cache_inhabitance: bool,
  pub(crate) solver_mode: SolverMode,
}
