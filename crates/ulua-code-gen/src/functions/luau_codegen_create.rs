use alloc::boxed::Box;
use core::ptr::null_mut;

use ulua_common::fint::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};
use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::initialize_execution_callbacks::initialize_execution_callbacks,
  records::{
    base_code_gen_context::BaseCodeGenContext,
    standalone_code_gen_context::StandaloneCodeGenContext,
  },
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

  // Safety: 契约保证 l 为存活 LuaState*。ctx 由 Box::new 就地构造，其 #[repr(C)] 首字段
  // base 与 Box 指针同址，故把 &mut ctx.base 当作 BaseCodeGenContext* 上转换为合法；
  // 以 null_mut() 传 global/mutex 对齐 C++ nullptr 形参；Box::into_raw 将所有权移交 ecb，
  // 移交前 ctx 在本函数内始终有效，state 关闭时经 ecb 回收。

  // cpp CodeGen/src/CodeGenContext.cpp:455-463:
  //   auto codeGenContext =
  //       std::make_unique<StandaloneCodeGenContext>(blockSize, maxTotalSize, nullptr, nullptr);
  //   if (!codeGenContext->initHeaderFunctions())
  //       return;
  //   initializeExecutionCallbacks(L, codeGenContext.release());
  // Default 占位值全为 null/空容器，构造函数以 ptr::write 整体覆盖（等价 make_unique+ctor）。
  // Safety: 依上契约——ctor 为 unsafe 方法，实参为 Copy 值/空指针，ctx 存活于块内。
  let mut ctx = unsafe {
    let mut ctx = Box::new(StandaloneCodeGenContext::default());
    ctx.standalone_code_gen_context_standalone_code_gen_context(
      block_size,
      max_total_size,
      null_mut(),
      null_mut(),
    );
    ctx
  };

  // init_header_functions 为 safe 方法，仅操作存活的 ctx 自身。
  if !ctx.base.init_header_functions() {
    return; // cpp: unique_ptr 析构释放；此处 Box drop 等价
  }

  // cpp: codeGenContext.release() —— 所有权移交 ecb，
  // state 关闭时经 ecb.close → StandaloneCodeGenContext::onCloseState 销毁。
  // base 为 #[repr(C)] 首字段，地址与 Box 指针一致（cpp 隐式基类上转换）。
  let base: *mut BaseCodeGenContext = &mut ctx.base;
  let _ = Box::into_raw(ctx);
  // Safety: 依入口契约——l 为存活 LuaState*；base 指向刚移交所有权的 ctx，直至 state 关闭。
  unsafe { initialize_execution_callbacks(l, base) };
}
