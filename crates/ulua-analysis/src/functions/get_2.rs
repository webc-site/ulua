//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeUtils.h:217:get_2`
//! Source: `Analysis/include/Luau/TypeUtils.h:217-228` (hand-ported)

/// C++ `template<typename A, typename B, typename Ty> TryPair<const A*, const B*> get2(Ty one, Ty two)`.
use core::ptr::null;

use crate::{
  functions::get_type_utils::{GetThroughId, get_optional_ty},
  records::try_pair::TryPair,
};
pub fn get2<A, B, Ty>(one: Ty, two: Ty) -> TryPair<*const A, *const B>
where
  A: GetThroughId<Ty>,
  B: GetThroughId<Ty>,
  Ty: Copy,
{
  let a = unsafe { get_optional_ty::<A, Ty>(Some(one)) };
  let b = unsafe { get_optional_ty::<B, Ty>(Some(two)) };

  if !a.is_null() && !b.is_null() {
    TryPair {
      first: a,
      second: b,
    }
  } else {
    TryPair {
      first: null(),
      second: null(),
    }
  }
}
