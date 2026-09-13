use crate::{
  enums::unify_result::UnifyResult,
  functions::are_compatible::are_compatible,
  records::{unifier_2::Unifier2, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn unify_union_type_type_id(
    &mut self,
    sub_union: &UnionType,
    super_ty: TypeId,
  ) -> UnifyResult {
    let mut result = UnifyResult::Ok;

    for sub_option in sub_union.options.iter() {
      if unsafe { are_compatible(*sub_option, super_ty) } {
        result &= self.unify_type_id_type_id(*sub_option, super_ty);
      }
    }

    result
  }
}
