use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type, is_prim::is_nil},
  records::union_type::UnionType,
  type_aliases::type_variant::TypeVariantMember,
};

/// 对照 C++ `returnFirstNonnullOptionOfType`（AutocompleteCore.cpp）：
/// 取联合类型中首个非 nil 的 T 选项；存在第二个非 nil 选项则整体不可约。
/// 全部经由安全 API（引用、迭代器、`?`），tv 由 `&UnionType` 保证非空；
/// 生命周期取 `'static`：TypeId 指向 TypeArena 分配，与 C++ 指针语义一致
/// （同 functions/get_type.rs 惯用法）。
pub fn return_first_nonnull_option_of_type<T: TypeVariantMember + 'static>(
  utv: &UnionType,
) -> Option<&'static T> {
  let mut ret: Option<&'static T> = None;

  // C++ `for (TypeId subTy : utv)`——UnionTypeIterator 展平嵌套 union 并
  // follow,裸遍历 options 会漏掉嵌套成员。
  for sub_ty in begin_union_type(utv) {
    if is_nil(sub_ty) {
      continue;
    }

    // C++: get<T> 为空 → 存在非 T 选项 → 整体不可约
    let ftv = get_type::get::<T>(follow_type::follow(sub_ty))?;

    if ret.is_some() {
      return None;
    }

    ret = Some(ftv);
  }

  ret
}
