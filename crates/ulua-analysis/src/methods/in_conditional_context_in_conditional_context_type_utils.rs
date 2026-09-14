use crate::{
  enums::type_context::TypeContext, records::in_conditional_context::InConditionalContext,
};

impl InConditionalContext {
  /// # Safety
  /// 调用方须保证 `c` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn new(c: *mut TypeContext, new_value: TypeContext) -> Self {
    let old_value = unsafe { *c };
    unsafe {
      *c = new_value;
    }
    Self {
      type_context: c,
      old_value,
    }
  }
}
