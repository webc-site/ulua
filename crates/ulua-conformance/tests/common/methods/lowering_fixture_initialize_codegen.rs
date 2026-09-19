use core::{ffi::c_char, slice::from_raw_parts, str::from_utf8};

use ulua_code_gen::{
  enums::host_metamethod::HostMetamethod,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

use crate::common::functions::{
  userdata_access::userdata_access, userdata_access_bytecode_type::userdata_access_bytecode_type,
  userdata_metamethod::userdata_metamethod,
  userdata_metamethod_bytecode_type::userdata_metamethod_bytecode_type,
  userdata_namecall::userdata_namecall,
  userdata_namecall_bytecode_type::userdata_namecall_bytecode_type, vector_access::vector_access,
  vector_access_bytecode_type::vector_access_bytecode_type, vector_namecall::vector_namecall,
  vector_namecall_bytecode_type::vector_namecall_bytecode_type,
};

#[inline]
unsafe fn member_to_str(ptr: *const c_char, len: usize) -> &'static str {
  if ptr.is_null() || len == 0 {
    ""
  } else {
    let bytes = unsafe { from_raw_parts(ptr as *mut u8, len) };
    from_utf8(bytes).unwrap_or("")
  }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vector_access_bytecode_type_callback(
  member: *const c_char,
  member_length: usize,
) -> u8 {
  let m = unsafe { member_to_str(member, member_length) };
  vector_access_bytecode_type(m)
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vector_namecall_bytecode_type_callback(
  member: *const c_char,
  member_length: usize,
) -> u8 {
  let m = unsafe { member_to_str(member, member_length) };
  vector_namecall_bytecode_type(m)
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vector_access_callback(
  builder: *mut IrBuilder,
  member: *const c_char,
  member_length: usize,
  result_reg: i32,
  source_reg: i32,
  pcpos: i32,
) -> bool {
  let m = unsafe { member_to_str(member, member_length) };
  unsafe { vector_access(&mut *builder, m, result_reg, source_reg, pcpos) }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vector_namecall_callback(
  builder: *mut IrBuilder,
  member: *const c_char,
  member_length: usize,
  arg_res_reg: i32,
  source_reg: i32,
  params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  let m = unsafe { member_to_str(member, member_length) };
  unsafe {
    vector_namecall(
      &mut *builder,
      m,
      arg_res_reg,
      source_reg,
      params,
      results,
      pcpos,
    )
  }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_access_bytecode_type_callback(
  r#type: u8,
  member: *const c_char,
  member_length: usize,
) -> u8 {
  let m = unsafe { member_to_str(member, member_length) };
  userdata_access_bytecode_type(r#type, m)
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_metamethod_bytecode_type_callback(
  lhs_ty: u8,
  rhs_ty: u8,
  method: HostMetamethod,
) -> u8 {
  userdata_metamethod_bytecode_type(lhs_ty, rhs_ty, method)
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_namecall_bytecode_type_callback(
  r#type: u8,
  member: *const c_char,
  member_length: usize,
) -> u8 {
  let m = unsafe { member_to_str(member, member_length) };
  userdata_namecall_bytecode_type(r#type, m)
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_access_callback(
  builder: *mut IrBuilder,
  r#type: u8,
  member: *const c_char,
  member_length: usize,
  result_reg: i32,
  source_reg: i32,
  pcpos: i32,
) -> bool {
  let m = unsafe { member_to_str(member, member_length) };
  unsafe { userdata_access(&mut *builder, r#type, m, result_reg, source_reg, pcpos) }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_metamethod_callback(
  builder: *mut IrBuilder,
  lhs_ty: u8,
  rhs_ty: u8,
  result_reg: i32,
  lhs: IrOp,
  rhs: IrOp,
  method: HostMetamethod,
  pcpos: i32,
) -> bool {
  unsafe {
    userdata_metamethod(
      &mut *builder,
      lhs_ty,
      rhs_ty,
      result_reg,
      lhs,
      rhs,
      method,
      pcpos,
    )
  }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_namecall_callback(
  builder: *mut IrBuilder,
  r#type: u8,
  member: *const c_char,
  member_length: usize,
  arg_res_reg: i32,
  source_reg: i32,
  params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  let m = unsafe { member_to_str(member, member_length) };
  unsafe {
    userdata_namecall(
      &mut *builder,
      r#type,
      m,
      arg_res_reg,
      source_reg,
      params,
      results,
      pcpos,
    )
  }
}
