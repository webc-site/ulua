use alloc::boxed::Box;
use core::ptr::null_mut;

use ulua_common::fint::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};
use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::options::CodeGenContextKind,
  functions::initialize_execution_callbacks::initialize_execution_callbacks,
  records::base_code_gen_context::BaseCodeGenContext,
};

/// cpp CodeGen/src/lcodegen.cpp:13 `luau_codegen_create` → `Luau::CodeGen::create(L)`。
///
/// # Safety
/// `l` 必须是有效且存活的 `LuaState` 指针（对齐 C API 调用契约）。
pub unsafe fn luau_codegen_create(l: *mut LuaState) {
  // cpp CodeGen/src/CodeGenContext.cpp:445:
  //   void create(LuaState* L)
  //   { return create(L, size_t(FInt::LuauCodeGenBlockSize),
  //                     size_t(FInt::LuauCodeGenMaxTotalSize), nullptr, nullptr); }
  let block_size = LuauCodeGenBlockSize.get() as usize;
  let max_total_size = LuauCodeGenMaxTotalSize.get() as usize;

  // cpp CodeGen/src/CodeGenContext.cpp:455-463:
  //   auto codeGenContext =
  //       std::make_unique<StandaloneCodeGenContext>(blockSize, maxTotalSize, nullptr, nullptr);
  //   if (!codeGenContext->initHeaderFunctions())
  //       return;
  //   initializeExecutionCallbacks(L, codeGenContext.release());
  // Rust 侧对应 Box::new + kind=Standalone（派生类收口为判别枚举）；
  // nullptr/nullptr 形参即「无自定义分配回调」——构造器以 `Option::None` 表达。
  let mut ctx = Box::new(BaseCodeGenContext::new(
    CodeGenContextKind::Standalone,
    block_size,
    max_total_size,
    None,
    null_mut(),
  ));
  // 回指接线必须在 Box 落位之后、所有权移交之前（`pin_shared_allocator` 契约）。
  ctx.pin_shared_allocator();

  // init_header_functions 为 safe 方法，仅操作存活的 ctx 自身；失败走 Box 正常 drop
  // （等价 cpp unique_ptr 出作用域释放）。
  if !ctx.init_header_functions() {
    return;
  }

  // cpp: codeGenContext.release() —— 所有权移交 ecb，
  // state 关闭时经 ecb.close → on_close_state 的配对 Box::from_raw 一次性回收。
  let ctx = Box::into_raw(ctx);
  // Safety: 依入口契约——l 为存活 LuaState*；ctx 指向刚移交所有权、直至 state 关闭存活的上下文。
  unsafe { initialize_execution_callbacks(l, ctx) };
}
