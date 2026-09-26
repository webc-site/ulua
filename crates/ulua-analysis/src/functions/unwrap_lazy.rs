//! Source: `Analysis/src/Type.cpp` (Type.cpp:37-55, hand-ported)

use alloc::string::String;
use std::panic::panic_any;

use crate::{
  functions::get_type::get,
  records::{internal_compiler_error::InternalCompilerError, lazy_type::LazyType},
  type_aliases::type_id::TypeId,
};
/// # Safety
/// `ltv` 须指向 type arena 中存活、地址不移动的 `LazyType`（非空、对齐）；本函数会就地写其
/// `unwrapped` 字段并可能触发 `unwrap` 回调，故调用方须保证对该节点的独占可变访问，且节点比
/// 返回的 `TypeId` 使用期长寿。单线程、无并发写别名。对应 C++ `static TypeId unwrapLazy(LazyType* ltv)`
/// (`cpp/Analysis/src/Type.cpp:37`)。
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

    if get::<LazyType>(unwrapped).is_some() {
      panic_any(InternalCompilerError::new(
        String::from("Lazy Type cannot resolve to another Lazy Type"),
        None,
        None,
      ));
    }

    unwrapped
  }
}
