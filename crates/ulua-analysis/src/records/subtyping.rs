use core::ptr::null_mut;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, normalizer::Normalizer,
    subtyping_result::SubtypingResult, type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, type_pair_hash::TypePairHash,
  },
  type_aliases::{
    seen_set_subtyping::SeenSet, seen_type_pack_set::SeenTypePackSet, type_id::TypeId,
  },
};

#[derive(Debug, Clone)]
pub struct Subtyping {
  // 以下五个字段原为照抄 C++ `NotNull<T>*` 的裸指针，现收敛为句柄：目标均为
  // 宿主（Frontend/TypeChecker2 等）进程级独占持有的单例，恒非空。
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) arena: Handle<TypeArena>,
  // `Option`：TypeChecker2/NonStrictTypeChecker 构造期先置空，`wire_self_pointers`
  // 在对象落位后接线（原 null_mut 哨兵的等价表达）。
  pub(crate) normalizer: Option<Handle<Normalizer>>,
  pub(crate) type_function_runtime: Handle<TypeFunctionRuntime>,
  pub(crate) ice_reporter: Handle<InternalErrorReporter>,
  pub(crate) limits: TypeCheckLimits,
  pub(crate) unique_types: *const DenseHashSet<TypeId>,
  pub(crate) seen_types: SeenSet,
  pub(crate) seen_packs: SeenTypePackSet,
  pub(crate) result_cache: DenseHashMap<(TypeId, TypeId), SubtypingResult, TypePairHash>,
}

impl Subtyping {
  /// 已接线 Normalizer 的可变借用。null 仅存在于 `wire_self_pointers` 之前
  /// （与原裸指针时代同一使用契约），故此处为确定性 panic 而非 UB。
  pub(crate) fn normalizer_mut(&self) -> &mut Normalizer {
    self
      .normalizer
      .expect("Subtyping.normalizer 须在使用前由 wire_self_pointers 接线")
      .get_mut()
  }

  /// 接线期 normalizer 的裸指针值：供仍以 `*mut Normalizer` 互操作的调用点，
  /// 接线前为 null（与原字段直读行为等价）。
  pub(crate) fn normalizer_ptr(&self) -> *mut Normalizer {
    self.normalizer.map(|h| h.as_ptr()).unwrap_or(null_mut())
  }
}
