use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_vm::records::proto::Proto;

use crate::enums::code_gen_flags::CodeGenFlags;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn gather_functions_helper(
  results: &mut Vec<*mut Proto>,
  proto: *mut Proto,
  flags: u32,
  has_native_functions: bool,
  root: bool,
) {
  let proto_ref = unsafe { &*proto };

  if results.len() <= proto_ref.bytecodeid as usize {
    results.resize(proto_ref.bytecodeid as usize + 1, null_mut());
  }

  if !results[proto_ref.bytecodeid as usize].is_null() {
    return;
  }

  let lpf_native_function = 1 << 0;
  let lpf_native_cold = 1 << 1;

  let should_gather = if has_native_functions {
    !root && (proto_ref.flags as u32 & lpf_native_function) != 0
  } else {
    (proto_ref.flags as u32 & lpf_native_cold) == 0
      || (flags & (CodeGenFlags::CodeGenColdFunctions as u32)) != 0
  };

  if should_gather {
    results[proto_ref.bytecodeid as usize] = proto;
  }

  for i in 0..proto_ref.sizep as usize {
    let child_proto = unsafe { *proto_ref.p.add(i) };
    unsafe { gather_functions_helper(results, child_proto, flags, has_native_functions, false) };
  }
}
