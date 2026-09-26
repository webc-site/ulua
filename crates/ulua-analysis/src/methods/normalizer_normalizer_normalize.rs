use alloc::collections::BTreeMap;
use core::ptr::null_mut;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, normalizer::Normalizer,
    type_arena::TypeArena, unifier_shared_state::UnifierSharedState,
  },
};
impl Normalizer {
  /// C++ `Normalizer::Normalizer(TypeArena*, NotNull<BuiltinTypes>,
  /// NotNull<UnifierSharedState>, SolverMode, bool cacheInhabitance = false)`
  /// (`Analysis/src/Normalize.cpp:858`). Owned constructor: the five
  /// collaborators are stored; every cache is default-empty (the
  /// `DenseHashMap`s take their `nullptr` empty-key, matching the C++
  /// member initializers `{nullptr}` / `{{nullptr, nullptr}}`).
  ///
  /// `arena`/`shared_state` 用 `Option<Handle>`：arena 的 null 是「模块外归一化」
  /// 有语义哨兵；`TypeChecker` 构造时两者都要等对象在 Box 中定址后才接线
  /// （对应 C++ 成员初始化列表里 `normalizer(nullptr, …, NotNull{&unifierState})`
  /// 的两段式互引），绝不能在此断言非空。
  pub fn new(
    arena: Option<Handle<TypeArena>>,
    builtin_types: Handle<BuiltinTypes>,
    shared_state: Option<Handle<UnifierSharedState>>,
    solver_mode: SolverMode,
    cache_inhabitance: bool,
  ) -> Self {
    Normalizer {
      cached_normals: BTreeMap::new(),
      cached_intersections: BTreeMap::new(),
      cached_unions: BTreeMap::new(),
      cached_type_ids: BTreeMap::new(),
      cached_is_inhabited: DenseHashMap::default(),
      cached_is_inhabited_intersection: DenseHashMap::default(),
      fuel: None,
      arena,
      builtin_types,
      shared_state,
      cache_inhabitance,
      solver_mode,
    }
  }

  pub fn normalizer_type_arena_not_null_builtin_types_not_null_unifier_shared_state_solver_mode_bool(
    &mut self,
    arena: Option<Handle<TypeArena>>,
    builtin_types: Handle<BuiltinTypes>,
    shared_state: Option<Handle<UnifierSharedState>>,
    solver_mode: SolverMode,
    cache_inhabitance: bool,
  ) {
    self.arena = arena;
    self.builtin_types = builtin_types;
    self.shared_state = shared_state;
    self.cache_inhabitance = cache_inhabitance;
    self.solver_mode = solver_mode;
  }

  /// 已接线的 arena 可变视图：归一化期 arena 必已由 `TypeChecker` 的 check
  /// 入口（cpp `checkWithoutRecursionCheck`）/构造实参接线；null 哨兵只出现
  /// 在「模块外归一化」上报路径（该路径先经 `arena.is_none()` 判空短路），
  /// 走到这里属契约违例，确定性 panic 而非 UB。
  pub(crate) fn wired_arena_mut(&self) -> &mut TypeArena {
    self
      .arena
      .expect("Normalizer.arena 使用前必须已接线（模块外归一化应先走判空上报分支）")
      .get_mut()
  }

  /// arena 的句柄形态（判空即 panic，与 `arena_mut` 的接线断言同一契约），
  /// 供仍以 `Handle<TypeArena>` 为参数的协作者直接使用，免经裸指针往返。
  pub(crate) fn arena_handle(&self) -> Handle<TypeArena> {
    self
      .arena
      .expect("Normalizer.arena 使用前必须已接线（模块外归一化应先走判空上报分支）")
  }

  /// 已接线 shared_state 的共享视图（cpp `NotNull<UnifierSharedState>` 成员，
  /// TypeChecker 在 Box 定址后即接线；使用前断言接线，等价原裸指针解引用）。
  pub(crate) fn shared_state_ref(&self) -> &UnifierSharedState {
    self
      .shared_state
      .expect("Normalizer.shared_state 使用前必须已接线")
      .get()
  }

  /// 已接线 shared_state 的可变视图：读改计数器（recursion_count 等）。
  pub(crate) fn shared_state_mut(&self) -> &mut UnifierSharedState {
    self
      .shared_state
      .expect("Normalizer.shared_state 使用前必须已接线")
      .get_mut()
  }

  /// shared_state 的裸指针形态（含 null），供仍存 `*mut UnifierSharedState`
  /// 字段的接口（Unifier/外部透传）与就地判空语义使用。
  pub(crate) fn shared_state_ptr(&self) -> *mut UnifierSharedState {
    self.shared_state.map_or(null_mut(), |state| state.as_ptr())
  }
}
