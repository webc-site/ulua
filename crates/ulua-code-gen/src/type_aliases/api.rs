use alloc::string::String;
use core::ffi::{c_char, c_void};

pub use ulua_vm::{
  records::lua_state::LuaState, type_aliases::luau_fast_function::LuauFastFunction,
};

use crate::{
  enums::host_metamethod::HostMetamethod,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// 宿主代码页分配回调（cpp `CodeAllocator.h` 的 `void* (*allocationCallback)` 槽）：
/// `context` 是宿主自持的 opaque user-data（本 crate 从不解引用，原样交回），
/// 「旧块/新块」两侧由 `(pointer, size)` 配对表达，缺席侧为 `null` + `size == 0`。
///
/// # Safety（实现方契约）
/// 注册点（`CodeAllocator::code_allocator_usize_usize_allocation_callback_void` /
/// `BaseCodeGenContext::new` / `create_shared_code_gen_context*`）之后，被调实现必须：
/// - 只把 `context` 当作不透明值使用，不解引用为具体类型（类型由注册方与实现方自行约定）；
/// - `new_pointer` 非空时须为按 `new_size` 映射、可安全 `mprotect`/`munmap` 的页对齐区，
///   `new_pointer` 为 null 时不得解引用；
/// - `old_pointer` 非空时须为此前同一回调以 `new_pointer` 交付且仍存活的块基址；
/// - 不得跨回调持有本 crate 交出的指针超过其配对释放（`free_pages`）之后的时间。
pub type AllocationCallback = unsafe extern "C-unwind" fn(
  context: *mut c_void,
  old_pointer: *mut c_void,
  old_size: usize,
  new_pointer: *mut c_void,
  new_size: usize,
);

/// 汇编转储注解回调（cpp `AssemblyBuilder*::annotator`）：`context` 为宿主自持 opaque
/// user-data，`result` 是本 crate 借出的、仅在本次调用期间存活的追加式缓冲视图。
///
/// # Safety（实现方契约）
/// - 只把 `context` 当不透明值使用；
/// - 只能向 `result` 追加文本，不得保存其引用越过本次调用；
/// - `fid`/`instpos` 为被注解指令的定位信息，宿主不得据此写回本 crate 状态。
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

/// 宿主 userdata 类型名映射回调（cpp `CodeGenContext.h` 的 `UserdataRemapperCallback`）：
/// 名称以 C 字符串形态（字符指针 + 显式长度）跨过宿主 ABI 边界，是本 crate 唯一保留
/// C 字符入参的字符串点——字节码 loader 交出的就是 NUL 结尾的 `TString` 载荷，
/// 长度由 `name_length` 界定（review.md §3：C 字符只准出现在真 FFI 边界）。
///
/// # Safety（实现方契约）
/// - `name` 在本次调用期间指向 `name_length` 字节可读缓冲（末位为 NUL 终止符，
///   由 `proto_views::string_constant` 的界内性保证）；实现方不得保存该指针；
/// - 只把 `context` 当不透明值使用，其具体类型由 `set_userdata_remapper` 的注册方
///   与实现方自行约定；
/// - 返回值须小于 `LBC_TYPE_TAGGED_USERDATA_END - LBC_TYPE_TAGGED_USERDATA_BASE`
///   才有意义（越界由 `userdata_remapper_wrap` 折回 `LBC_TYPE_USERDATA`）。
pub type UserdataRemapperCallback =
  unsafe extern "C-unwind" fn(context: *mut c_void, name: *const c_char, name_length: usize) -> u8;
