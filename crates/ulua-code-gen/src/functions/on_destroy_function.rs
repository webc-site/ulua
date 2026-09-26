use core::ptr::null_mut;

use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::{
  functions::get_code_gen_context::get_code_gen_context,
  records::shared_code_gen_context::SharedCodeGenContext,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_destroy_function(l: *mut LuaState, proto: *mut Proto) {
  // Safety: 先判空 l/proto 后返回, 二者其后均活; ctx 经 is_null 判定为活上下文。
  // (*proto).execdata 判非空后传给 SharedCodeGenContext::on_destroy_function 归还引用计数,
  // 满足其对有效 execdata 的前置条件。随后清空 execdata/exectarget 并令 codeentry=code
  // 均为对本活 Proto 字段的合法写(code 为该 proto 自身字节码数组)。以下窄块统一援引。
  if l.is_null() || proto.is_null() {
    return;
  }

  let ctx = unsafe { get_code_gen_context(l) };

  // Safety: 依上契约——短路次序与拆分前一致；execdata 判非空后交 callee 归还引用计数。
  if unsafe { !ctx.is_null() && !(*proto).execdata.is_null() } {
    unsafe { SharedCodeGenContext::on_destroy_function((*proto).execdata) };
  }

  // Safety: 依上契约——均为对本活 Proto 字段的合法写。
  unsafe {
    (*proto).execdata = null_mut();
    (*proto).exectarget = 0;
    (*proto).codeentry = (*proto).code;
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn on_destroy_function_export(l: *mut LuaState, proto: *mut Proto) {
  // Safety: 导出 C ABI 入口原样转发 l/proto 给同契约 unsafe fn on_destroy_function; 该内部
  // 函数对 l/proto 判空早返回并仅在 execdata 非空时使用, 满足被调前置条件。
  unsafe { on_destroy_function(l, proto) };
}
