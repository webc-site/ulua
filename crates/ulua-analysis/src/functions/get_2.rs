//! Source: `Analysis/include/Luau/TypeUtils.h:217-228` (hand-ported)

/// C++ `template<typename A, typename B, typename Ty> TryPair<const A*, const B*> get2(Ty one, Ty two)`
/// 的 Rust 惯用形式：双方均命中时返回引用对，否则 `None`。
use crate::functions::get_type_utils::{GetThroughId, get_optional_ty};

pub fn get2<'a, A, B, Ty>(one: Ty, two: Ty) -> Option<(&'a A, &'a B)>
where
  A: GetThroughId<Ty>,
  B: GetThroughId<Ty>,
  Ty: Copy,
{
  // SAFETY: 契约同 C++——one/two 指向 arena 中存活的类型节点。
  let a = unsafe { get_optional_ty::<A, Ty>(Some(one)) };
  // SAFETY: 同上。
  let b = unsafe { get_optional_ty::<B, Ty>(Some(two)) };

  if a.is_null() || b.is_null() {
    return None;
  }
  // SAFETY: 非空指针指向 arena 节点，TypeId 存续期内稳定有效。
  Some((unsafe { &*a }, unsafe { &*b }))
}
