use core::ffi::c_void;

use crate::{
  functions::{
    get_code_gen_context::get_code_gen_context, userdata_remapper_wrap::userdata_remapper_wrap,
  },
  type_aliases::{lua_state::LuaState, userdata_remapper_callback::UserdataRemapperCallback},
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
  // get_code_gen_context 内部已处理 l / global / context 的判空。
  // Safety: get_code_gen_context 内部判空 l/global/context 并可能返回 null，本行仅按契约转发
  // l（合法 LuaState*），随后 !is_null() 分支才解引用，故此调用只需 l 有效。
  let codegen_ctx = unsafe { get_code_gen_context(l) };
  if codegen_ctx.is_null() {
    return;
  }

  // Safety: codegen_ctx 依上方判空为有效 BaseCodeGenContext；调用方按 `# Safety` 契约
  // 保证 l（及 l->global）存活有效。以下窄块统一援引本契约。
  unsafe {
    (*codegen_ctx).userdata_remapping_context = context;
    // C++ 直接存函数指针（`userdataRemapper = cb`，可为 nullptr）；Rust 字段为
    // `Option<UserdataRemapperCallback>`（None 即 nullptr），故直接转发调用方的 Option。
    (*codegen_ctx).userdata_remapper = cb;
  }

  // C++: l->global->ecb.gettypemapping = cb ? userdataRemapperWrap : nullptr;
  // cb 为 None 时必须撤下跳板（清空为 None），与 cpp 三元表达式逐支一致。
  let global = unsafe { (*l).global };
  if !global.is_null() {
    unsafe {
      (*global).ecb.gettypemapping = if cb.is_some() {
        Some(userdata_remapper_wrap)
      } else {
        None
      };
    }
  }
}
