use alloc::{alloc::dealloc, vec::Vec};
use core::{alloc::Layout, ptr::drop_in_place};

use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::{
  functions::{
    gather_functions_helper::gather_functions_helper,
    initialize_execution_callbacks::initialize_execution_callbacks,
  },
  records::shared_code_gen_context::SharedCodeGenContext,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create(l: *mut LuaState, code_gen_context: *mut SharedCodeGenContext) {
  // Safety: `SharedCodeGenContext` 与 `BaseCodeGenContext` 已同型（派生类收口为
  // kind 判别），指针即 ecb 契约要求的 base 上下文指针，无任何转型；其存活性和
  // 类型正确性由本函数头 `/// # Safety` 约定的调用方前提保证，原样转发给接线函数。
  unsafe { initialize_execution_callbacks(l, code_gen_context) };
}

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

/// 沿原型链收集待编译 `Proto`：返回按 `bytecodeid` 索引的稀疏表，`None` 为空槽
/// （对齐 cpp 的 `nullptr` 哨兵语义），元素借用随 `root` 的存活契约（整段编译
/// 会话内由 VM 持有，见 `proto_views` 模块文档）。
pub(crate) fn gather_functions<'a>(
  root: &'a Proto,
  flags: u32,
  has_native_functions: bool,
) -> Vec<Option<&'a Proto>> {
  let mut results: Vec<Option<&'a Proto>> = Vec::new();
  gather_functions_helper(&mut results, root, flags, has_native_functions, true);
  results
}
