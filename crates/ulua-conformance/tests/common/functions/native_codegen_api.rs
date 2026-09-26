//! native codegen / SharedCodeAllocator 用例的安全门面（review.md §0/§2）。
//!
//! [`ulua_code_gen`] 的引用计数与 execdata 头读写是 FFI/裸内存边界的固有 `unsafe`；
//! 本模块把每个动作各自收口成一次性 `unsafe`，用例侧只做 safe 调用。集中契约：
//!
//! - **模块裸指针 [`NativeModule`] 契约**：`mod_*` 系列收 `*const NativeModule`，要求
//!   该指针由本用例此前经 [`NativeModuleRef::native_module_ref_get`] 取得、且被其引用
//!   计数（或手工 `add_ref`）保活至使用点——与直接 `(*p).method()` 完全同语义。
//! - **`data`/`code` 切片契约**：空切片按 `(null, 0)` 透传（复刻旧 `null(), 0` 哨兵），
//!   非空切片按 `as_ptr()/len()` 透传，长度与指针有效性由被调例程负责。
//! - **proto 头契约**：`patch_*`/`proto_*` 收 [`NativeProtoExecDataPtr`] 或其
//!   `instruction_offsets` 裸指针，要求指向 codegen 分配内合法的 execdata。

use alloc::vec::Vec;
use core::{ffi::c_int, ptr::null};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_code_gen::{
  functions::{
    compile_internal::compile_internal,
    create_code_gen_context::create,
    get_native_proto_exec_data_header_native_proto_exec_data::{
      get_native_proto_exec_data_header, get_native_proto_exec_data_header_mut,
    },
  },
  records::{
    compilation_options::CompilationOptions, compilation_result::CompilationResult,
    compilation_stats::CompilationStats, native_module::NativeModule,
    native_module_ref::NativeModuleRef, shared_code_allocator::SharedCodeAllocator,
    shared_code_gen_context::SharedCodeGenContext,
  },
  type_aliases::{module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr},
};
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{functions::luau_load::luau_load, records::lua_state::LuaState};

/// 可空切片 → `(ptr, len)` 哨兵收口：空切片译成 `(null, 0)`，与旧写法逐字同形。
fn bytes_ptr_len(bytes: &[u8]) -> (*const u8, usize) {
  if bytes.is_empty() {
    (null(), 0)
  } else {
    (bytes.as_ptr(), bytes.len())
  }
}

// ---------------------------------------------------------------------------
// NativeModuleRef / NativeModule 引用计数
// ---------------------------------------------------------------------------

/// 读引用计数（收模块裸指针）。
pub fn mod_refcount(m: *const NativeModule) -> usize {
  // Safety: `m` 由本用例此前取得且仍存活（模块级契约）。
  unsafe { &*m }.native_module_get_refcount()
}

/// `native_module_add_ref`（收模块裸指针）。
pub fn mod_add_ref(m: *const NativeModule) -> usize {
  // Safety: 同 [`mod_refcount`]。
  unsafe { &*m }.native_module_add_ref()
}

/// `native_module_add_refs(count)`（收模块裸指针）。
pub fn mod_add_refs(m: *const NativeModule, count: usize) -> usize {
  // Safety: 同 [`mod_refcount`]。
  unsafe { &*m }.native_module_add_refs(count)
}

/// `release()`（收模块裸指针）。
pub fn mod_release(m: *const NativeModule) -> usize {
  // Safety: 同 [`mod_refcount`]。
  unsafe { &*m }.release()
}

/// `native_module_get_module_base_address`（收模块裸指针）。
pub fn mod_base_address(m: *const NativeModule) -> *const u8 {
  // Safety: 同 [`mod_refcount`]。
  unsafe { &*m }.native_module_get_module_base_address()
}

/// `native_module_try_get_native_proto`（收模块裸指针；缺省返回 null，由用例判空）。
pub fn mod_try_get_proto(m: *const NativeModule, bytecode_id: u32) -> *const u32 {
  // Safety: 同 [`mod_refcount`]。
  unsafe { &*m }.native_module_try_get_native_proto(bytecode_id)
}

/// 以 `mr` 所持模块为目标的 [`mod_refcount`] 便捷封装。
pub fn refcount_of(mr: &NativeModuleRef) -> usize {
  mod_refcount(mr.native_module_ref_get())
}

/// 经 `*mut NativeModuleRef` 自搬运赋值（move-assign-through-pointer），逐字复刻
/// 旧 `unsafe { let moved = ...mut(&mut *p); (*p).operator_assign(moved); }` 序列。
pub fn self_move_assign(p: *mut NativeModuleRef) {
  // Safety: `p` 指向本用例作用域内、尚未被释放或再借用的 `NativeModuleRef`。
  unsafe {
    let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut *p);
    (*p).native_module_ref_operator_assign(moved);
  }
}

// ---------------------------------------------------------------------------
// SharedCodeAllocator 插入
// ---------------------------------------------------------------------------

/// `insert_anonymous_native_module`：空 `data`/`code` 切片按 `(null, 0)` 透传。
pub fn insert_anonymous(
  allocator: &mut SharedCodeAllocator,
  native_protos: Vec<NativeProtoExecDataPtr>,
  data: &[u8],
  code: &[u8],
) -> NativeModuleRef {
  let (data_ptr, data_len) = bytes_ptr_len(data);
  let (code_ptr, code_len) = bytes_ptr_len(code);
  // Safety: `native_protos` 元素均指向合法 execdata；`data`/`code` 满足上面透传契约。
  unsafe {
    allocator.insert_anonymous_native_module(native_protos, data_ptr, data_len, code_ptr, code_len)
  }
}

/// `get_or_insert_native_module`，仅回交模块引用（旧调用点一律取 `.0`）。
pub fn get_or_insert(
  allocator: &mut SharedCodeAllocator,
  module_id: &ModuleId,
  native_protos: Vec<NativeProtoExecDataPtr>,
  data: &[u8],
  code: &[u8],
) -> NativeModuleRef {
  let (data_ptr, data_len) = bytes_ptr_len(data);
  let (code_ptr, code_len) = bytes_ptr_len(code);
  // Safety: 同 [`insert_anonymous`]，另 `module_id` 为合法键。
  unsafe {
    allocator
      .get_or_insert_native_module(
        module_id,
        native_protos,
        data_ptr,
        data_len,
        code_ptr,
        code_len,
      )
      .0
  }
}

// ---------------------------------------------------------------------------
// NativeProtoExecData 头读写
// ---------------------------------------------------------------------------

/// 为新建 proto 打补丁：写 `bytecode_id`、`entry_offset_or_address` 与前两条指令偏移。
pub fn patch_proto(
  proto: &NativeProtoExecDataPtr,
  bytecode_id: u32,
  entry: *const u8,
  words: [u32; 2],
) {
  // Safety: `proto` 指向合法 execdata 分配，header 紧接其 instruction_offsets 之前，
  // 且该分配至少含两条 instruction offset（create 时按 capacity 预置）。
  unsafe {
    let header = get_native_proto_exec_data_header_mut(proto.as_ptr());
    (*header).bytecode_id = bytecode_id;
    (*header).entry_offset_or_address = entry;
    *proto.as_ptr().add(0) = words[0];
    *proto.as_ptr().add(1) = words[1];
  }
}

/// 仅写 header 的 `bytecode_id`（保留其余字段），复刻旧单字段补丁块。
pub fn patch_proto_bytecode_id(proto: &NativeProtoExecDataPtr, bytecode_id: u32) {
  // Safety: 同 [`patch_proto`]（此处仅改 `bytecode_id` 一个字段）。
  unsafe {
    (*get_native_proto_exec_data_header_mut(proto.as_ptr())).bytecode_id = bytecode_id;
  }
}

/// 读 proto 的 `bytecode_id`（`proto` 为 `instruction_offsets` 裸指针）。
pub fn proto_bytecode_id(proto: *const u32) -> u32 {
  // Safety: `proto` 指向合法 execdata 的 instruction_offsets 首元素。
  unsafe { (*get_native_proto_exec_data_header(proto)).bytecode_id }
}

/// 读 proto 的 `entry_offset_or_address`。
pub fn proto_entry(proto: *const u32) -> *const u8 {
  // Safety: 同 [`proto_bytecode_id`]。
  unsafe { (*get_native_proto_exec_data_header(proto)).entry_offset_or_address }
}

/// 读 proto 的第 `i` 条指令偏移。
pub fn proto_word(proto: *const u32, i: usize) -> u32 {
  // Safety: `proto.add(i)` 落在该 proto 的 instruction_offsets 数组内。
  unsafe { *proto.add(i) }
}

/// 模块基址按字节偏移（复刻旧 `base.add(n)`）：`n` 落在分配内的地址计算。
pub fn base_add(base: *const u8, offset: usize) -> *const u8 {
  // Safety: `base` 为 codegen 分配的模块基址，`base + offset` 仍在同一分配内。
  unsafe { base.add(offset) }
}

// ---------------------------------------------------------------------------
// 共享 codegen 上下文 / 编译-加载链
// ---------------------------------------------------------------------------

/// `create`：为状态 `l` 挂接共享 codegen 上下文。
pub fn codegen_create_shared(l: *mut LuaState, ctx: *mut SharedCodeGenContext) {
  // Safety: `l` 存活；`ctx` 由本用例的 SharedContextRef 保活且覆盖 `l` 的生命周期。
  unsafe { create(l, ctx) }
}

/// safe `compile`（缺省选项，对应 cpp `luau_compile(source, size, nullptr, &size)`）
/// → owned [`Vec<u8>`]：cpp 的 malloc/free 所有权契约在 Rust 端由产物所有权消解，
/// 不留悬垂缓冲。
pub fn compile_bytecode(source: &[u8]) -> Vec<u8> {
  compile(
    source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  )
}

/// `luau_load` 收口：把已物化的字节码切片加载进 `l`，返回状态码。
pub fn load_bytes(l: *mut LuaState, chunkname: &str, bytes: &[u8]) -> c_int {
  // Safety: `l` 存活；`chunkname`/`bytes` 为合法借用。
  unsafe { luau_load(l, chunkname, bytes, 0) }
}

/// `compile_internal`（带 `module_id` 与统计出参）收口：返回主结果与统计。
pub fn compile_native_with_stats(
  l: *mut LuaState,
  module_id: &ModuleId,
  idx: c_int,
  options: &CompilationOptions,
) -> (CompilationResult, CompilationStats) {
  let mut stats = CompilationStats::default();
  // Safety: `l` 存活、`idx` 指向已加载 proto；`module_id`/`options`/`stats` 均为合法借用。
  let result = unsafe { compile_internal(&Some(*module_id), l, idx, options, &mut stats) };
  (result, stats)
}
