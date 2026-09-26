use crate::{
  records::{
    scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult,
  },
  type_aliases::type_id::TypeId,
};

impl Subtyping {
  pub fn try_semantic_subtyping(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: *mut Scope,
    original: &mut SubtypingResult,
  ) -> SubtypingResult {
    // C++ 中 normalize 失败返回空指针，进入 normalized 重载后命中等价分支
    // `{false, true}`（非子类型且归一化过于复杂）
    let (sub_norm, super_norm) = (
      // Safety: `self.normalizer_ptr()` 在 Subtyping 的 `subtyping_not_null_*` 唯一装配点
      // 一次性接线为指向驱动本次 subtyping 全程存活的 `&mut Normalizer`（非空、对齐）；
      // try_semantic_subtyping 只在装配完成后经 is_subtype/is_covariant 到达，sub_ty 为
      // 存活 arena TypeId。try_normalize 取 &mut，其借用随返回（Option<Arc<_>>）结束。
      { self.normalizer_mut().try_normalize(sub_ty) },
      // Safety: 同上——normalizer 非空存活；tuple 两元素顺序求值，上一条 &mut 借用已
      // 结束，本条重建独占可变借用无别名冲突。
      { self.normalizer_mut().try_normalize(super_ty) },
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
