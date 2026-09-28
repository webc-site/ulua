use alloc::string::String;
use core::ffi::{c_char, c_void};

pub use ulua_vm::{
  records::lua_state::LuaState, type_aliases::luau_fast_function::LuauFastFunction,
};

use crate::{
  enums::host_metamethod::HostMetamethod,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub type AllocationCallback = unsafe extern "C-unwind" fn(
  context: *mut c_void,
  old_pointer: *mut c_void,
  old_size: usize,
  new_pointer: *mut c_void,
  new_size: usize,
);

pub type AnnotatorFn = Option<
  unsafe extern "C-unwind" fn(context: *mut c_void, result: &mut String, fid: i32, instpos: i32),
>;

/// 宿主 userdata 属性访问 IR 生成 hook（cpp `userdataAccess` 的惯用 Rust 形态）。
pub type HostUserdataAccessHandler = Option<
  fn(
    builder: &mut IrBuilder,
    r#type: u8,
    member: &[u8],
    result_reg: i32,
    source_reg: i32,
    pcpos: i32,
  ) -> bool,
>;

/// 宿主 userdata 元方法的字节码类型推断 hook。
pub type HostUserdataMetamethodBytecodeType =
  Option<fn(lhs_ty: u8, rhs_ty: u8, method: HostMetamethod) -> u8>;

/// 宿主 userdata 元方法 IR 生成 hook（cpp `userdataMetamethod` 的惯用 Rust 形态）。
pub type HostUserdataMetamethodHandler = Option<
  fn(
    builder: &mut IrBuilder,
    lhs_ty: u8,
    rhs_ty: u8,
    result_reg: i32,
    lhs: IrOp,
    rhs: IrOp,
    method: HostMetamethod,
    pcpos: i32,
  ) -> bool,
>;

/// 宿主 userdata 命名调用 IR 生成 hook（cpp `userdataNamecall` 的惯用 Rust 形态）。
pub type HostUserdataNamecallHandler = Option<
  fn(
    builder: &mut IrBuilder,
    r#type: u8,
    member: &[u8],
    arg_res_reg: i32,
    source_reg: i32,
    params: i32,
    results: i32,
    pcpos: i32,
  ) -> bool,
>;

/// 宿主 userdata 操作（属性访问/命名调用）的字节码类型推断 hook。
pub type HostUserdataOperationBytecodeType = Option<fn(r#type: u8, member: &[u8]) -> u8>;

/// 宿主 vector 属性访问 IR 生成 hook（cpp `vectorAccess` 的惯用 Rust 形态）：
/// 成员名为 `k[]` 字符串常量字节切片，接管时返回 true。
pub type HostVectorAccessHandler = Option<
  fn(builder: &mut IrBuilder, member: &[u8], result_reg: i32, source_reg: i32, pcpos: i32) -> bool,
>;

/// 宿主 vector 命名调用 IR 生成 hook（cpp `vectorNamecall` 的惯用 Rust 形态）。
pub type HostVectorNamecallHandler = Option<
  fn(
    builder: &mut IrBuilder,
    member: &[u8],
    arg_res_reg: i32,
    source_reg: i32,
    params: i32,
    results: i32,
    pcpos: i32,
  ) -> bool,
>;

/// 宿主 vector 操作（属性访问/命名调用）的字节码类型推断 hook。
pub type HostVectorOperationBytecodeType = Option<fn(member: &[u8]) -> u8>;

// CodeGen 视角下的 `LuaState`——VM 自己的类型，并非不透明替身
// （CodeGen 直接对真实 VM 状态执行操作）。

// `LuauFastFunction` 单一来源为 ulua-vm 的
// [`ulua_vm::type_aliases::api::LuauFastFunction`]，此处复导出
// 供 JIT 消费面（`NativeContext.luau_f_table` 槽位、A64/X64 FASTCALL lowering
// 经 `offset_of!`/`size_of!` 读取表槽位布局）使用。

pub type UserdataRemapperCallback =
  unsafe extern "C-unwind" fn(context: *mut c_void, name: *const c_char, name_length: usize) -> u8;
