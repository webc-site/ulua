use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_nil::is_nil},
  records::union_type::UnionType,
  type_aliases::type_variant::TypeVariantMember,
};

/// 对照 C++ `returnFirstNonnullOptionOfType`（TypeFunctionRuntime.cpp）：
/// 取联合类型中首个非 nil 的 T 选项；存在第二个非 nil 选项则整体不可约。
/// 全部经由安全 API（引用、迭代器、`?`），tv 由 `&UnionType` 保证非空；
/// 生命周期取 `'static`：TypeId 指向 TypeArena 分配，与 C++ 指针语义一致
/// （同 functions/get_type_alt_j.rs 惯用法）。
pub fn return_first_nonnull_option_of_type<T: TypeVariantMember + 'static>(
  utv: &UnionType,
) -> Option<&'static T> {
  let mut ret: Option<&'static T> = None;

  for &sub_ty in &utv.options {
    if is_nil(sub_ty) {
      continue;
    }

    // C++: get<T> 为空 → 存在非 T 选项 → 整体不可约
    let ftv = get_type_id::<T>(follow_type_id(sub_ty))?;

    if ret.is_some() {
      return None;
    }

    ret = Some(ftv);
  }

  ret
}
