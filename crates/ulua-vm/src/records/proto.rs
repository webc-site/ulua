use core::ffi::{c_int, c_void};

use crate::{
  records::{
    feedback_vector_slot::FeedbackVectorSlot, g_cheader::GCheader, gc_object::GCObject,
    loc_var::LocVar, t_string::tstring,
  },
  type_aliases::{instruction::Instruction, t_value::TValue},
};

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
  pub abslineinfo: *mut c_int,
  pub locvars: *mut LocVar,
  pub upvalues: *mut *mut tstring,
  pub source: *mut tstring,

  pub debugname: *mut tstring,
  pub debuginsn: *mut u8,

  pub typeinfo: *mut u8,

  pub userdata: *mut c_void,

  pub gclist: *mut GCObject,

  pub sizecode: c_int,
  pub sizep: c_int,
  pub sizelocvars: c_int,
  pub sizeupvalues: c_int,
  pub sizek: c_int,
  pub sizelineinfo: c_int,
  pub linegaplog2: c_int,
  pub linedefined: c_int,
  pub bytecodeid: c_int,
  pub sizetypeinfo: c_int,

  pub feedbackvec: *mut FeedbackVectorSlot,
  pub feedbackvecsize: u32,
  pub funid: u32,
}
