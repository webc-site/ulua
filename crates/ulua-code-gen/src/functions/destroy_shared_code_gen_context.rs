use alloc::alloc::dealloc;
use core::{alloc::Layout, ptr::drop_in_place};

use crate::records::shared_code_gen_context::SharedCodeGenContext;

/// # Safety
///
/// 本函数仅由 Rust 侧直接调用（如测试 Drop 守卫），执行手动内存释放。
/// 调用方必须保证 `code_gen_context` 由匹配的分配函数创建，
/// 且本调用之后不再使用它。
pub unsafe fn destroy_shared_code_gen_context(code_gen_context: *const SharedCodeGenContext) {
  if !code_gen_context.is_null() {
    let ptr = code_gen_context as *mut SharedCodeGenContext;
    // Safety: 契约保证 code_gen_context 为匹配分配函数产出、此后不再使用的
    // SharedCodeGenContext 指针，且 !is_null() 已判空；先 drop_in_place 跑析构，再以
    // Layout::new::<SharedCodeGenContext>()（与分配端一致）释放整块。
    unsafe {
      drop_in_place(ptr);
      dealloc(ptr as *mut u8, Layout::new::<SharedCodeGenContext>());
    }
  }
}
