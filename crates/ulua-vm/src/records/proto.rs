use core::ffi::c_void;

use crate::{
  records::{
    feedback_vector_slot::FeedbackVectorSlot, g_cheader::GCheader, gc_object::GCObject,
    loc_var::LocVar, t_string::tstring,
  },
  type_aliases::{instruction::Instruction, t_value::TValue},
};

/// 函数原型（GC 对象）。`#[repr(C)]` 与字段顺序是 ulua-code-gen JIT 的 ABI 契约：
/// x64/arm64 发射器按 `offset_of!(Proto, numparams/is_vararg/k/code/execdata/exectarget)` 直接寻址，
/// 且 `ldp` 依赖 `k`→`code`、`execdata`→`exectarget` 两对字段相邻，禁止重排或插删。
///
/// 裸指针字段均为非拥有句柄（§11 的 Vec/arena 化属后续 VM 阶段）：
/// `k`/`code`/`p`/`lineinfo`/`abslineinfo`/`locvars`/`upvalues`/`debuginsn`/`typeinfo`/`feedbackvec`
/// 指向按对应 `size*` 计数分配、随本 Proto 一起由 `lua_f_freeproto` 释放的子数组；
/// `source`/`debugname` 为存活 TString；`gclist` 为 sweep 链侵入式指针；
/// `codeentry` 是 `code` 的只读入口别名，`execdata`/`userdata` 由 JIT/code-gen 宿主持有。
#[derive(Debug)]
#[repr(C)]
pub struct Proto {
  pub hdr: GCheader,

  pub nups: u8,
  pub numparams: u8,
  pub is_vararg: u8,
  pub maxstacksize: u8,
  pub flags: u8,

  pub k: *mut TValue,
  pub code: *mut Instruction,
  pub p: *mut *mut Proto,
  pub codeentry: *const Instruction,

  pub execdata: *mut c_void,
  pub exectarget: usize,

  pub lineinfo: *mut u8,
  pub abslineinfo: *mut i32,
  pub locvars: *mut LocVar,
  pub upvalues: *mut *mut tstring,
  pub source: *mut tstring,

  pub debugname: *mut tstring,
  pub debuginsn: *mut u8,

  pub typeinfo: *mut u8,

  pub userdata: *mut c_void,

  pub gclist: *mut GCObject,

  pub sizecode: i32,
  pub sizep: i32,
  pub sizelocvars: i32,
  pub sizeupvalues: i32,
  pub sizek: i32,
  pub sizelineinfo: i32,
  pub linegaplog2: i32,
  pub linedefined: i32,
  pub bytecodeid: i32,
  pub sizetypeinfo: i32,

  pub feedbackvec: *mut FeedbackVectorSlot,
  pub feedbackvecsize: u32,
  pub funid: u32,
  /// cpp: `Proto::cost`（`cpp/VM/src/lobject.h:414`）
  pub cost: u64,
}
