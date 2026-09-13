//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:37:unwrap_lazy`
//! Source: `Analysis/src/Type.cpp` (Type.cpp:37-55, hand-ported)

use alloc::string::String;
use std::panic::panic_any;

use crate::{
  functions::get_type_alt_j::get,
  records::{internal_compiler_error::InternalCompilerError, lazy_type::LazyType},
  type_aliases::type_id::TypeId,
};
/// # Safety
/// 调用方须保证 `ltv` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn unwrap_lazy(ltv: *mut LazyType) -> TypeId {
  unsafe {
    let mut unwrapped: TypeId = (*ltv).unwrapped;

    if !unwrapped.is_null() {
      return unwrapped;
    }

    if let Some(unwrap) = (*ltv).unwrap {
      unwrap(&mut *ltv);
    }
    unwrapped = (*ltv).unwrapped;

    if unwrapped.is_null() {
      panic_any(InternalCompilerError::new(
        String::from("Lazy Type didn't fill in unwrapped type field"),
        None,
        None,
      ));
    }

    if !get::<LazyType>(unwrapped).is_none() {
      panic_any(InternalCompilerError::new(
        String::from("Lazy Type cannot resolve to another Lazy Type"),
        None,
        None,
      ));
    }

    unwrapped
  }
}
