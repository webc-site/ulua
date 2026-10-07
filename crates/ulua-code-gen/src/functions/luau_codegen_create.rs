use alloc::boxed::Box;
use core::ptr::null_mut;

use ulua_common::fint::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};
use ulua_vm::{
  functions::lua_setsafeenv::lua_setsafeenv, macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::LuaState,
};

use crate::{
  enums::options::CodeGenContextKind,
  functions::initialize_execution_callbacks::initialize_execution_callbacks,
  records::base_code_gen_context::BaseCodeGenContext,
};

/// 上下文装配共用路径：safeenv 提升与否按形态分流，其余（Standalone 上下文、
/// gate header、code allocator、ecb 回调注册）两形态完全同构。
///
/// # Safety
/// `l` 必须是有效且存活的 `LuaState` 指针（对齐 C API 调用契约）。
unsafe fn codegen_create_impl(l: *mut LuaState, set_safeenv: bool) {
  // 激活 safeenv（cpp linit.cpp:110 在 luaL_sandbox 里做，本仓库刻意挪到 JIT 启用点）：
  // safeenv=0 时生成码的 implicit CHECK_SAFE_ENV guard 会把一切带全局读取的模块
  // 逐调用弹回解释器（实测 nbody 74ms→17ms、fasta -50% 的根因即此）。
  if set_safeenv {
    unsafe {
      lua_setsafeenv(l, LUA_GLOBALSINDEX, 1);
    }
  }

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

/// cpp CodeGen/src/lcodegen.cpp:13 `luau_codegen_create` → `Luau::CodeGen::create(L)`。
///
/// # Safety
/// `l` 必须是有效且存活的 `LuaState` 指针（对齐 C API 调用契约）。
pub unsafe fn luau_codegen_create(l: *mut LuaState) {
  // Safety: 契约透传，safeenv 提升 1（JIT 形态语义，见 `codegen_create_impl` 注）。
  unsafe { codegen_create_impl(l, true) };
}

/// FORN trace 层形态的上下文装配（阶段二 PoC）：jit 关 + `LuauJitFornTrace` 开
/// ——与 [`luau_codegen_create`] 唯一差异是 safeenv 保持 0；本形态装载期不编译
/// 任何 proto（trace 在运行期热度达标后录制/生成/安装，见 `trace_forn_registry`），
/// 上下文仅承载 ecb 槽注册与 trace 注册表。
///
/// 调用方须保证每 state 只装配一次（重复调用会覆盖 `ecb.context`，旧上下文失配
/// 成死账）——rt 侧以 `LuaInner` 幂等护栏收口。
///
/// # Safety
/// `l` 必须是有效且存活的 `LuaState` 指针（对齐 C API 调用契约）。
pub unsafe fn luau_codegen_create_forn_trace_only(l: *mut LuaState) {
  // Safety: 契约透传，safeenv 保持 0（解释器旁路形态语义，零扰动）。
  unsafe { codegen_create_impl(l, false) };
}
