use core::ffi::c_void;

use ulua_vm::records::lua_state::lua_State;

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
  l: *mut lua_State,
  code_gen_context: *mut BaseCodeGenContext,
) {
  CODEGEN_ASSERT!(!code_gen_context.is_null());

  unsafe {
    let ecb = &mut (*(*l).global).ecb;

    ecb.context = code_gen_context as *mut c_void;
    ecb.close = Some(on_close_state_export);
    ecb.destroy = Some(on_destroy_function_export);
    ecb.enter = Some(on_enter_export);
    ecb.disable = Some(on_disable_export);
    ecb.getmemorysize = Some(get_memory_size_export);
    ecb.getcounterdata = Some(get_counter_data_export);
  }
}
