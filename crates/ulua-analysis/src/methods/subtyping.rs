//! `subtyping` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::ToString;
use core::ptr::null;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseHasher};

use crate::{
  enums::type_field::TypeField,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, normalizer::Normalizer,
    property_type_path::Property as PathProperty, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, type_ids::TypeIds, type_pair_hash::TypePairHash,
  },
  type_aliases::{component::Component, type_id::TypeId, type_pack_id::TypePackId},
};

impl DenseHasher<(TypeId, TypeId)> for TypePairHash {
  fn hash(&self, x: &(TypeId, TypeId)) -> usize {
    let left = self.hash_one_type_id(x.0);
    let right = self.hash_one_type_id(x.1);
    left ^ (right << 1)
  }
}
// C++ `TypePairHash` provides `operator()` overloads for both `std::pair<TypeId,
// TypeId>` and `std::pair<TypePackId, TypePackId>` (used by `SeenTypePackSet`).
impl DenseHasher<(TypePackId, TypePackId)> for TypePairHash {
  fn hash(&self, x: &(TypePackId, TypePackId)) -> usize {
    self.hash_one_type_pack_id(x.0) ^ (self.hash_one_type_pack_id(x.1) << 1)
  }
}
impl Subtyping {
  pub fn cache(
    &mut self,
    _env: &mut SubtypingEnvironment,
    result: SubtypingResult,
    sub_ty: TypeId,
    super_ty: TypeId,
  ) -> SubtypingResult {
    let p = (sub_ty, super_ty);

    if result.is_cacheable {
      *self.result_cache.get_or_insert(p) = result.clone();
    }

    result
  }
}

impl Subtyping {
  pub fn maybe_update_bounds(
    &mut self,
    here: TypeId,
    there: TypeId,
    bounds_to_update: &mut TypeIds,
    first_bounds_to_check: &TypeIds,
    second_bounds_to_check: &TypeIds,
  ) {
    let mut bounds_changed = false;

    if !first_bounds_to_check.empty() {
      for t in first_bounds_to_check.order.iter() {
        let t = *t;
        if t != here {
          bounds_to_update.insert_type_id(t);
          bounds_changed = true;
        }
      }
    }

    if !bounds_changed && !second_bounds_to_check.empty() {
      for t in second_bounds_to_check.order.iter() {
        let t = *t;
        if t != here {
          bounds_to_update.insert_type_id(t);
          bounds_changed = true;
        }
      }
    }

    if !bounds_changed && here != there {
      bounds_to_update.insert_type_id(there);
    }
  }
}

// `Subtyping` 推理路径 `Component` 构造简写：`is_covariant_with`（现行）与
// `is_covariant_with_deprecated`（旧求解器旗标路径）两侧同款使用，此前两文件
// 各抄一份，收口于此。

/// 表属性读写路径分量（cpp `SubtypingReason` 的 `Property{readTy,writeTy}` 形态）。
pub(crate) fn path_property(name: &str, is_read: bool) -> Component {
  Component::Property(PathProperty {
    name: name.to_string(),
    is_read,
  })
}
/// indexer 结果类型分量（cpp `TypeField::IndexResult`）。
pub(crate) fn index_result_component() -> Component {
  Component::TypeField(TypeField::IndexResult)
}

impl Subtyping {
  /// C++ `Subtyping::Subtyping(NotNull<BuiltinTypes>, NotNull<TypeArena>,
  /// NotNull<Normalizer>, NotNull<TypeFunctionRuntime>,
  /// NotNull<InternalErrorReporter>)` — sets the five collaborators; every
  /// other member is default-constructed. 形参全为受检句柄/引用：`normalizer`
  /// 的 `None` 对应 TypeChecker2/NonStrict 构造期先置空的接线前状态（原 null
  /// 哨兵的可空表达）；`Handle` 即 crate 的「可变别名许可」句柄标记。
  pub fn subtyping_owned(
    builtin_types: Handle<BuiltinTypes>,
    type_arena: Handle<TypeArena>,
    normalizer: Option<Handle<Normalizer>>,
    type_function_runtime: &TypeFunctionRuntime,
    ice_reporter: &InternalErrorReporter,
  ) -> Self {
    Subtyping {
      builtin_types,
      arena: type_arena,
      normalizer,
      type_function_runtime: Handle::from_ref(type_function_runtime),
      ice_reporter: Handle::from_ref(ice_reporter),
      limits: TypeCheckLimits::default(),
      unique_types: null(),
      seen_types: DenseHashMap::default(),
      seen_packs: DenseHashMap::default(),
      result_cache: DenseHashMap::default(),
    }
  }
}

impl Subtyping {
  pub fn try_semantic_subtyping(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: &Scope,
    original: &mut SubtypingResult,
  ) -> SubtypingResult {
    // C++ 中 normalize 失败返回空指针，进入 normalized 重载后命中等价分支
    // `{false, true}`（非子类型且归一化过于复杂）
    let (sub_norm, super_norm) = (
      // Safety: `self.normalizer_ptr()` 在 Subtyping 的 `subtyping_not_null_*` 唯一装配点
      // 一次性接线为指向驱动本次 subtyping 全程存活的 `&mut Normalizer`（非空、对齐）；
      // try_semantic_subtyping 只在装配完成后经 is_subtype/is_covariant 到达，sub_ty 为
      // 存活 arena TypeId。try_normalize 取 &mut，其借用随返回（Option<Arc<_>>）结束。
      self.normalizer_mut().try_normalize(sub_ty),
      // Safety: 同上——normalizer 非空存活；tuple 两元素顺序求值，上一条 &mut 借用已
      // 结束，本条重建独占可变借用无别名冲突。
      self.normalizer_mut().try_normalize(super_ty),
    );
    let (Some(sub_norm), Some(super_norm)) = (sub_norm, super_norm) else {
      return SubtypingResult::too_complex();
    };
    let mut semantic = self
            .is_covariant_with_subtyping_environment_shared_ptr_normalized_type_shared_ptr_normalized_type_not_null_scope(
                env,
                &sub_norm,
                &super_norm,
                scope,
            );

    if semantic.normalization_too_complex {
      semantic
    } else if semantic.is_subtype {
      semantic.reasoning.clear();
      semantic
    } else {
      original.clone()
    }
  }
}
