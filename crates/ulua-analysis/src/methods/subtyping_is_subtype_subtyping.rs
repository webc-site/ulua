use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::subtyping_suppression_policy::SubtypingSuppressionPolicy,
  functions::{follow_type, get_type},
  records::{
    generic_bounds::GenericBounds, generic_type::GenericType,
    mapped_generic_environment::MappedGenericEnvironment, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Subtyping {
  pub fn is_subtype_type_id_type_id_not_null_scope(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut env = SubtypingEnvironment {
      parent: null_mut(),
      mapped_generics: DenseHashMap::default(),
      mapped_generic_packs: MappedGenericEnvironment {
        frames: Vec::new(),
        current_scope_index: None,
      },
      substitutions: DenseHashMap::default(),
      seen_set_cache: DenseHashMap::default(),
      iteration_count: 0,
    };

    let result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
      &mut env, sub_ty, super_ty, scope,
    );

    if result.normalization_too_complex {
      if result.is_cacheable {
        // cpp `resultCache[{subTy, superTy}] = result`：operator[] 覆盖旧条目
        self.result_cache.insert((sub_ty, super_ty), result.clone());
      }
      return result;
    }

    // cpp: 协变求解结束后所有 mapped generic 的 bounds 必须已弹空，否则说明
    // 有泛型环境泄漏到了外层，缓存结果就不再成立。
    for (_, bounds) in env.mapped_generics.iter() {
      LUAU_ASSERT!(bounds.is_empty());
    }

    if result.is_cacheable {
      self.result_cache.insert((sub_ty, super_ty), result.clone());
    }

    result
  }

  /// C++:
  /// ```cpp
  /// SubtypingResult Subtyping::isSubtype(TypePackId subTp, TypePackId superTp, NotNull<Scope> scope,
  ///     const std::vector<TypeId>& bindableGenerics)
  /// {
  ///     const std::vector<TypePackId> bindableGenericPacks;
  ///     return isSubtype(subTp, superTp, scope, bindableGenerics, bindableGenericPacks);
  /// }
  /// ```
  pub fn is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    scope: *mut Scope,
    bindable_generics: &[TypeId],
  ) -> SubtypingResult {
    let bindable_generic_packs: Vec<TypePackId> = Vec::new();
    self.is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id_vector_type_pack_id(
      sub_tp,
      super_tp,
      scope,
      bindable_generics,
      &bindable_generic_packs,
    )
  }

  /// C++:
  /// ```cpp
  /// SubtypingResult Subtyping::isSubtype(
  ///     TypePackId subTp, TypePackId superTp, NotNull<Scope> scope,
  ///     const std::vector<TypeId>& bindableGenerics,
  ///     const std::vector<TypePackId>& bindableGenericPacks)
  /// {
  ///     SubtypingEnvironment env;
  ///     for (TypeId g : bindableGenerics)
  ///         env.mappedGenerics[follow(g)] = {SubtypingEnvironment::GenericBounds{}};
  ///     env.mappedGenericPacks.pushFrame(bindableGenericPacks);
  ///     SubtypingResult result = isCovariantWith(env, subTp, superTp, scope);
  ///     for (TypeId bg : bindableGenerics) {
  ///         bg = follow(bg);
  ///         LUAU_ASSERT(env.mappedGenerics.contains(bg));
  ///         if (const std::vector<SubtypingEnvironment::GenericBounds>* bounds = env.mappedGenerics.find(bg)) {
  ///             LUAU_ASSERT(bounds->size() == 1);
  ///             if (bounds->empty()) continue;
  ///             if (const GenericType* r#gen = get<GenericType>(bg))
  ///                 result.andAlso(checkGenericBounds(bounds->back(), env, scope, r#gen->name));
  ///         }
  ///     }
  ///     return result;
  /// }
  /// ```
  pub(crate) fn is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id_vector_type_pack_id(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    scope: *mut Scope,
    bindable_generics: &[TypeId],
    bindable_generic_packs: &[TypePackId],
  ) -> SubtypingResult {
    let mut env = SubtypingEnvironment {
      parent: null_mut(),
      mapped_generics: DenseHashMap::default(),
      mapped_generic_packs: MappedGenericEnvironment {
        frames: Vec::new(),
        current_scope_index: None,
      },
      substitutions: DenseHashMap::default(),
      seen_set_cache: DenseHashMap::default(),
      iteration_count: 0,
    };

    for &g in bindable_generics.iter() {
      *env.mapped_generics.get_or_insert(follow_type::follow(g)) =
        alloc::vec![GenericBounds::default()];
    }

    env.mapped_generic_packs.push_frame(bindable_generic_packs);

    let mut result = unsafe {
      self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
        &mut env, sub_tp, super_tp, scope,
      )
    };

    for &bg in bindable_generics.iter() {
      let bg = follow_type::follow(bg);

      LUAU_ASSERT!(env.mapped_generics.contains(&bg));

      // Clone the bounds out so the immutable borrow of `env` is released
      // before the `&mut env` call to `checkGenericBounds`.
      let last_bounds = match env.mapped_generics.find(&bg) {
        Some(bounds) => {
          // Bounds should have exactly one entry
          LUAU_ASSERT!(bounds.len() == 1);
          // 双写合一：`is_empty()` continue 与 `last().unwrap()` 并为一条
          // `let Some(..) else continue`，Some 直接绑定。
          let Some(last_bound) = bounds.last().cloned() else {
            continue;
          };
          last_bound
        }
        None => continue,
      };

      if let Some(r#gen) = get_type::get::<GenericType>(bg) {
        let generic_name = r#gen.name.clone();
        let bounds_result =
          self.subtyping_check_generic_bounds(&last_bounds, &mut env, scope, &generic_name);
        result.and_also(bounds_result, SubtypingSuppressionPolicy::Any);
      }
    }

    result
  }
}
