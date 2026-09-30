//! Source: `Analysis/include/Luau/TypeUtils.h:217-228` (hand-ported)

/// C++ `template<typename A, typename B, typename Ty> TryPair<const A*, const B*> get2(Ty one, Ty two)`
/// 的 Rust 惯用形式：双方均命中时返回引用对，否则 `None`。
use crate::functions::get_type_utils::{GetThroughId, get_optional_ty};

pub fn get2<'a, A, B, Ty>(one: Ty, two: Ty) -> Option<(&'a A, &'a B)>
where
  A: GetThroughId<Ty> + 'static,
  B: GetThroughId<Ty> + 'static,
  Ty: Copy,
{
  let a = get_optional_ty::<A, Ty>(Some(one))?;
  let b = get_optional_ty::<B, Ty>(Some(two))?;
  Some((a, b))
}
