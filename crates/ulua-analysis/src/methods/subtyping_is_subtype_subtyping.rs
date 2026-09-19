use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  records::{
    mapped_generic_environment::MappedGenericEnvironment, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
  },
  type_aliases::type_id::TypeId,
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
      mapped_generics: DenseHashMap::new(null()),
      mapped_generic_packs: MappedGenericEnvironment {
        frames: Vec::new(),
        current_scope_index: None,
      },
      substitutions: DenseHashMap::new(null()),
      seen_set_cache: DenseHashMap::new((null(), null())),
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
}
