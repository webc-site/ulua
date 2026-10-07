use core::ffi::c_void;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::{
    get_counter_data::get_counter_data_export, get_memory_size::get_memory_size_export,
    on_close_state::on_close_state_export, on_destroy_function::on_destroy_function_export,
    on_disable::on_disable_export, on_enter::on_enter_export,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::base_code_gen_context::BaseCodeGenContext,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn initialize_execution_callbacks(
  l: *mut LuaState,
  code_gen_context: *mut BaseCodeGenContext,
) {
  CODEGEN_ASSERT!(!code_gen_context.is_null());

  // Safety: 契约保证 l 为存活 LuaState* 且 l->global 已初始化，入口 CODEGEN_ASSERT 亦验
  // code_gen_context 非空；&mut (*(*l).global).ecb 取得独占可变借用（串行装配无别名），
  // 写入的 ecb.* 均为本 crate 提供的合法 extern 回调，context 为判空后的 base 指针。
  unsafe {
    let ecb = &mut (*(*l).global).ecb;

    ecb.context = code_gen_context as *mut c_void;
    ecb.close = Some(on_close_state_export);
    ecb.destroy = Some(on_destroy_function_export);
    ecb.enter = Some(on_enter_export);
    ecb.disable = Some(on_disable_export);
    ecb.getmemorysize = Some(get_memory_size_export);
    ecb.getcounterdata = Some(get_counter_data_export);
    // FORN trace 层入口（阶段二 PoC）：仅 a64 编译目标安装——录制/生成/原生
    // 执行全链仅在 aarch64 生效，其余平台槽位 None（解释器零问询）。
    #[cfg(target_arch = "aarch64")]
    {
      use crate::functions::trace_forn_registry::{
        forn_trace_backedge_export, forn_trace_enter_export,
      };
      ecb.trace_forn_enter = Some(forn_trace_enter_export);
      // T2 回边计数慢路：IC 武装/解除全由 trace_forn_registry 维护
      ecb.trace_forn_backedge = Some(forn_trace_backedge_export);
    }
  }
}
