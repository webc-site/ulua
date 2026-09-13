use core::ptr::null_mut;

use ulua_vm::records::{lua_state::lua_State, proto::Proto};

use crate::{
  functions::get_code_gen_context::get_code_gen_context,
  records::shared_code_gen_context::SharedCodeGenContext,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_destroy_function(l: *mut lua_State, proto: *mut Proto) {
  unsafe {
    if l.is_null() || proto.is_null() {
      return;
    }

    let ctx = get_code_gen_context(l);
    if !ctx.is_null() && !(*proto).execdata.is_null() {
      SharedCodeGenContext::on_destroy_function((*proto).execdata);
    }

    (*proto).execdata = null_mut();
    (*proto).exectarget = 0;
    (*proto).codeentry = (*proto).code;
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_on_destroy_function")]
pub unsafe extern "C-unwind" fn on_destroy_function_export(l: *mut lua_State, proto: *mut Proto) {
  unsafe { on_destroy_function(l, proto) };
}
