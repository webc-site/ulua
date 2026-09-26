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

/// 把 IR hook 回调收到的成员名（指针 + 长度）转成 `&str`。
///
/// 指针指向 CodeGen 的临时缓冲（成员名来自 `IrBuilder`/字节码常量），只在本次回调期间
/// 有效，故返回值的生命周期由调用点决定（`'a` 由借用检查器在各调用处推断），不允许把
/// 结果保存到回调之外。
#[inline]
unsafe fn member_to_str<'a>(ptr: *const c_char, len: usize) -> &'a str {
  if ptr.is_null() || len == 0 {
    ""
  } else {
    let bytes = unsafe { from_raw_parts(ptr.cast::<u8>(), len) };
    from_utf8(bytes).unwrap_or("")
  }
}

/// 生成「字节码类型推断」hook 壳。
///
/// 对应 cpp `ConformanceIrHooks.h` 里返回 `LBC_TYPE_*` 的那组自由函数（
/// `vectorAccessBytecodeType` / `vectorNamecallBytecodeType` /
/// `userdataAccessBytecodeType` / `userdataNamecallBytecodeType`）。本仓库把这些钩子挂在
/// `HostIrHooks` 的 `extern "C-unwind" fn` 指针上，故每个钩子只能包一层壳；壳之间只差
/// 被调函数与可选的前置 `u8`（userdata 的字节码类型）形参，成员名的 (指针, 长度) → `&str`
/// 一律经 [`member_to_str`] 收口。
macro_rules! ir_hook_type_callback {
  (
    $name:ident,
    [$($lead:ident : $lead_ty:ty),* $(,)?],
    $callee:path $(,)?
  ) => {
    /// # Safety
    ///
    /// Pointer arguments must be valid, aligned, and properly initialized.
    pub unsafe extern "C-unwind" fn $name(
      $($lead: $lead_ty,)*
      member: *const c_char,
      member_length: usize,
    ) -> u8 {
      let m = unsafe { member_to_str(member, member_length) };
      $callee($($lead,)* m)
    }
  };
}

/// 生成「IR 指令生成」hook 壳。
///
/// 对应 cpp `ConformanceIrHooks.h` 里以 `IrBuilder&` 为首参、返回是否已处理的这组函数
/// （`vectorAccess` / `vectorNamecall` / `userdataAccess` / `userdataNamecall`）。
/// `$lead` 是成员名之前的前置形参（userdata 系列的 `u8` 字节码类型），`$tail` 是成员名
/// 之后的寄存器/pc 形参，两者都原样透传给 `$callee`。
macro_rules! ir_hook_builder_callback {
  (
    $name:ident,
    [$($lead:ident : $lead_ty:ty),* $(,)?],
    [$($tail:ident : $tail_ty:ty),* $(,)?],
    $callee:path $(,)?
  ) => {
    /// # Safety
    ///
    /// Pointer arguments must be valid, aligned, and properly initialized.
    pub unsafe extern "C-unwind" fn $name(
      builder: *mut IrBuilder,
      $($lead: $lead_ty,)*
      member: *const c_char,
      member_length: usize,
      $($tail: $tail_ty,)*
    ) -> bool {
      // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`builder` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
      let m = unsafe { member_to_str(member, member_length) };
      // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`builder` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
      unsafe { $callee(&mut *builder, $($lead,)* m, $($tail,)*) }
    }
  };
}

ir_hook_type_callback!(
  vector_access_bytecode_type_callback,
  [],
  vector_access_bytecode_type
);
ir_hook_type_callback!(
  vector_namecall_bytecode_type_callback,
  [],
  vector_namecall_bytecode_type
);
ir_hook_type_callback!(
  userdata_access_bytecode_type_callback,
  [r#type: u8],
  userdata_access_bytecode_type
);
ir_hook_type_callback!(
  userdata_namecall_bytecode_type_callback,
  [r#type: u8],
  userdata_namecall_bytecode_type
);

ir_hook_builder_callback!(
  vector_access_callback,
  [],
  [result_reg: i32, source_reg: i32, pcpos: i32],
  vector_access
);
ir_hook_builder_callback!(
  vector_namecall_callback,
  [],
  [
    arg_res_reg: i32,
    source_reg: i32,
    params: i32,
    results: i32,
    pcpos: i32
  ],
  vector_namecall
);
ir_hook_builder_callback!(
  userdata_access_callback,
  [r#type: u8],
  [result_reg: i32, source_reg: i32, pcpos: i32],
  userdata_access
);
ir_hook_builder_callback!(
  userdata_namecall_callback,
  [r#type: u8],
  [
    arg_res_reg: i32,
    source_reg: i32,
    params: i32,
    results: i32,
    pcpos: i32
  ],
  userdata_namecall
);

/// 唯一不带成员名的类型推断钩子：`method` 已是 `HostMetamethod`，没有 (指针, 长度) 需要
/// 收成 `&str`，因此不进宏。
///
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

/// metamethod 的 IR 生成壳：参数是 `IrOp` / `HostMetamethod`，没有成员名可收成 `&str`，
/// 与 [`ir_hook_builder_callback!`] 的形态不同，故手写。
///
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
  // Safety: `builder` 由 IR 钩子契约保证在本次回调期间存活且对齐，`&mut *builder` 的
  // 可变借用不超出本调用；其余实参均为按值传递的 IR 记录/标量。
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
