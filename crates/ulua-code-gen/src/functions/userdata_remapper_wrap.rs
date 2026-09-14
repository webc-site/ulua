use core::ffi::c_char;

use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;
use ulua_vm::records::lua_state::lua_State;

use crate::functions::get_code_gen_context::get_code_gen_context;

/// C trampoline installed into `global_State::ecb.gettypemapping` so the VM's
/// bytecode loader can map userdata type names through the registered remapper.
///
/// Mirrors `userdataRemapperWrap` (CodeGen/src/CodeGenContext.cpp):
/// ```cpp
/// static uint8_t userdataRemapperWrap(lua_State* l, const char* str, size_t len) {
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
/// `l` must be a valid `lua_State` pointer (or null, which is handled). This is
/// invoked by the VM through a function pointer, hence `extern "C"`.
pub unsafe extern "C-unwind" fn userdata_remapper_wrap(
  l: *mut lua_State,
  str: *const c_char,
  len: usize,
) -> u8 {
  unsafe {
    let codegen_ctx = get_code_gen_context(l);
    if !codegen_ctx.is_null() {
      let ctx = &*codegen_ctx;

      // The wrapper is only installed as `gettypemapping` when a remapper is
      // present, so `userdata_remapper` is `Some` here; this mirrors the C++
      // direct call and falls through to LBC_TYPE_USERDATA otherwise.
      if let Some(remapper) = ctx.userdata_remapper {
        let index = remapper(ctx.userdata_remapping_context, str, len);

        let base = LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8;
        let end = LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_END.0 as u8;

        if (index as u16) < (end as u16 - base as u16) {
          return base.wrapping_add(index);
        }
      }
    }

    LuauBytecodeType::LBC_TYPE_USERDATA.0 as u8
  }
}
