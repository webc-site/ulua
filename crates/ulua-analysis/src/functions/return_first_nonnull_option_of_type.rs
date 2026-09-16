use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_nil::is_nil},
  records::union_type::UnionType,
  type_aliases::type_variant::TypeVariantMember,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn return_first_nonnull_option_of_type<T: TypeVariantMember + 'static>(
  utv: &UnionType,
) -> Option<*const T> {
  let mut ret: Option<*const T> = None;

  for &sub_ty in &utv.options {
    if is_nil(sub_ty) {
      continue;
    }

    // C++: get<T> 为空 → 存在非 T 选项 → 整体不可约
    let ftv = get_type_id::<T>(follow_type_id(sub_ty))?;

    if ret.is_some() {
      return None;
    }

    ret = Some(ftv as *const T);
  }

  ret
}
