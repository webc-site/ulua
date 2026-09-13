use alloc::boxed::Box;
use core::fmt::{Debug, Formatter, Result};

use crate::{records::require_node::RequireNode, type_aliases::module_name_type::ModuleName};
#[repr(C)]
pub struct RequireSuggester {
  pub vtable: RequireSuggesterVtable,
}

#[derive(Clone, Copy)]
pub struct RequireSuggesterVtable {
  pub get_node:
    unsafe fn(*const RequireSuggester, name: &ModuleName) -> Option<Box<dyn RequireNode>>,
}

impl RequireSuggester {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn get_node(
    this: *const RequireSuggester,
    name: &ModuleName,
  ) -> Option<Box<dyn RequireNode>> {
    unsafe { ((*this).vtable.get_node)(this, name) }
  }
}

impl Debug for RequireSuggester {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("RequireSuggester").finish()
  }
}
