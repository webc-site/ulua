use core::ptr::null_mut;

use crate::records::{call_argument::CallArgument, ir_call_wrapper_x_64::IrCallWrapperX64};

pub fn ir_call_wrapper_x_64_find_non_interfering_argument(
  receiver: &IrCallWrapperX64,
) -> *mut CallArgument {
  for i in 0..receiver.arg_count {
    let arg_ptr = unsafe { receiver.args.as_ptr().add(i as usize) };
    let arg: &CallArgument = unsafe { &*arg_ptr };

    if arg.candidate
      && !receiver.interferes_with_active_sources(arg, i)
      && !receiver.interferes_with_operand(&receiver.func_op, arg.target.base)
    {
      return arg_ptr as *mut CallArgument;
    }
  }

  null_mut()
}
