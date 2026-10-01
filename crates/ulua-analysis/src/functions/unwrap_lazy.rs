//! Source: `Analysis/src/Type.cpp` (Type.cpp:37-55, hand-ported)

use alloc::string::String;
use std::panic::panic_any;

use crate::{
  functions::get_type::get,
  records::{internal_compiler_error::InternalCompilerError, lazy_type::LazyType},
  type_aliases::type_id::TypeId,
};
/// 就地解包 Lazy 节点并返回其 `unwrapped` TypeId。B 档契约前移（先例
/// abs_index/vmb1/mainthread）：原 `ltv` 裸指针「指向 type arena 中存活、地址不移动
/// 且独占可写」的契约改由 `&mut LazyType` 形参的引用有效性规则在调用点承载；
/// arena 节点地址稳定（bump 分配不移动），`&mut` 借用止于本函数返回。
/// 对应 C++ `static TypeId unwrapLazy(LazyType* ltv)` (`cpp/Analysis/src/Type.cpp:37`)。
pub fn unwrap_lazy(ltv: &mut LazyType) -> TypeId {
  let mut unwrapped: TypeId = ltv.unwrapped;

  if !unwrapped.is_null() {
    return unwrapped;
  }

  if let Some(unwrap) = ltv.unwrap {
    unwrap(ltv);
  }
  unwrapped = ltv.unwrapped;

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
