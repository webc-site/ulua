use crate::records::{lazy_type::LazyType, type_cloner::TypeCloner};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_lazy_type(&mut self, t: *mut LazyType) {
    // The `FragmentAutocompleteTypeCloner` override (Clone.cpp:541-544) does
    // not clone lazy types: it overrides `cloneChildren(LazyType*)` to a no-op.
    if self.skip_lazy_type_clone {
      return;
    }
    unsafe {
      if let Some(unwrapped) = (*t).unwrapped.as_ref() {
        (*t).unwrapped = self.shallow_clone_type_id(unwrapped as *const _ as *mut _);
      }
    }
  }
}
