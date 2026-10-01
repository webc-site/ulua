use core::ffi::c_void;

use crate::{
  functions::{
    get_code_gen_context::get_code_gen_context, userdata_remapper_wrap::userdata_remapper_wrap,
  },
  type_aliases::api::{LuaState, UserdataRemapperCallback},
};

/// 在代码生成上下文上设置 userdata remapper 回调与上下文。
///
/// 对应 `setUserdataRemapper`（CodeGen/src/CodeGenContext.cpp）：
/// ```cpp
/// void setUserdataRemapper(LuaState* l, void* context, UserdataRemapperCallback cb) {
///     if (BaseCodeGenContext* codegenCtx = getCodeGenContext(l)) {
///         codegenCtx->userdataRemappingContext = context;
///         codegenCtx->userdataRemapper = cb;
///         l->global->ecb.gettypemapping = cb ? userdataRemapperWrap : nullptr;
///     }
/// }
/// ```
///
/// # Safety
///
/// - `l` 必须是有效非空的 `LuaState` 指针。
/// - 全局状态（`l->global`）必须有效且已初始化。
/// - 代码生成上下文必须有效且已正确初始化。
/// - 回调 `cb`（若为 `Some`）必须是合法的函数指针。
#[inline]
pub unsafe fn set_userdata_remapper(
  l: *mut LuaState,
  context: *mut c_void,
  cb: Option<UserdataRemapperCallback>,
) {
  // Safety: get_code_gen_context 依其 `# Safety` 内部判空 l/global/context 并返回
  // 串行窗口内的独占借用；Some 分支的字段写入即本函数对上下文的全部消费。
  if let Some(ctx) = unsafe { get_code_gen_context(l) } {
    // C++ 直接存函数指针（`userdataRemapper = cb`，可为 nullptr）；Rust 字段为
    // `Option<UserdataRemapperCallback>`（None 即 nullptr），故直接转发调用方的 Option。
    ctx.userdata_remapping_context = context;
    ctx.userdata_remapper = cb;

    // C++: l->global->ecb.gettypemapping = cb ? userdataRemapperWrap : nullptr;
    // cb 为 None 时必须撤下跳板（清空为 None），与 cpp 三元表达式逐支一致。
    // Safety: 依本函数 `# Safety`——l 有效且 global 存活；此处仅写 ecb 的一个函数指针槽。
    let global = unsafe { (*l).global };
    if !global.is_null() {
      // Safety: 依上行判空。
      unsafe {
        (*global).ecb.gettypemapping = if cb.is_some() {
          Some(userdata_remapper_wrap)
        } else {
          None
        };
      }
    }
  }
}
