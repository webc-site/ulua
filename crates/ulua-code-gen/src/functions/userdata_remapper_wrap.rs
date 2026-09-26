use core::ffi::c_char;

use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;
use ulua_vm::records::lua_state::LuaState;

use crate::functions::get_code_gen_context::get_code_gen_context;

/// 安装进 `global_State::ecb.gettypemapping` 的 C trampoline，让 VM 的
/// 字节码 loader 能通过已注册的 remapper 映射 userdata 类型名。
///
/// 对应 `userdataRemapperWrap`（CodeGen/src/CodeGenContext.cpp）：
/// ```cpp
/// static uint8_t userdataRemapperWrap(LuaState* l, const char* str, size_t len) {
///     if (BaseCodeGenContext* codegenCtx = getCodeGenContext(l)) {
///         uint8_t index = codegenCtx->userdataRemapper(codegenCtx->userdataRemappingContext, str, len);
///         if (index < (LBC_TYPE_TAGGED_USERDATA_END - LBC_TYPE_TAGGED_USERDATA_BASE))
///             return LBC_TYPE_TAGGED_USERDATA_BASE + index;
///     }
///     return LBC_TYPE_USERDATA;
/// }
/// ```
///
/// # Safety
/// `l` 必须是有效的 `LuaState` 指针（或 null，已作处理）。本函数由
/// VM 经函数指针调用，因此是 `extern "C"`。
pub unsafe extern "C-unwind" fn userdata_remapper_wrap(
  l: *mut LuaState,
  str: *const c_char,
  len: usize,
) -> u8 {
  // Safety: extern "C-unwind" 由 VM 经函数指针调用，契约保证 l 为存活 LuaState* 或 null
  // （get_code_gen_context 内部判空）；codegen_ctx 判空后 &* 重建借用无别名（VM 串行调用），
  // remapper 为注册时提供的合法回调指针，str/len 由 VM 保证为可读字节串。以下窄块统一援引。
  let codegen_ctx = unsafe { get_code_gen_context(l) };
  if !codegen_ctx.is_null() {
    // Safety: 依上契约——判空后经 &* 重建只读借用，无别名（VM 串行调用）。
    let ctx = unsafe { &*codegen_ctx };

    // 只有在存在 remapper 时才会把本包装装为 `gettypemapping`，
    // 故此处 `userdata_remapper` 必为 `Some`；这与 C++ 的直接调用
    // 对应，其余情况落到 LBC_TYPE_USERDATA。
    if let Some(remapper) = ctx.userdata_remapper {
      // Safety: 依上契约——remapper 为注册期合法回调，str/len 由 VM 保证可读字节串。
      let index = unsafe { remapper(ctx.userdata_remapping_context, str, len) };

      let base = LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8;
      let end = LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_END.0 as u8;

      if (index as u16) < (end as u16 - base as u16) {
        return base.wrapping_add(index);
      }
    }
  }

  LuauBytecodeType::LBC_TYPE_USERDATA.0 as u8
}
