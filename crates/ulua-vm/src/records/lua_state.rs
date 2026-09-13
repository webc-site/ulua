use core::ffi::{c_int, c_void};

use crate::{
  records::{
    call_info::CallInfo, g_cheader::GCheader, gc_object::GcObject, global_state::global_State,
    lua_table::LuaTable, t_string::tstring, up_val::UpVal,
  },
  type_aliases::stk_id::StkId,
};

#[repr(C)]
#[derive(Debug)]
pub struct lua_State {
  pub hdr: GCheader,
  pub status: u8,
  pub activememcat: u8,
  pub isactive: bool,
  pub singlestep: bool,
  pub top: StkId,
  pub base: StkId,
  pub global: *mut global_State,
  pub ci: *mut CallInfo,
  pub stack_last: StkId,
  pub stack: StkId,
  pub end_ci: *mut CallInfo,
  pub base_ci: *mut CallInfo,
  pub stacksize: c_int,
  pub size_ci: c_int,
  pub n_ccalls: u16,
  pub base_ccalls: u16,
  pub cachedslot: c_int,
  pub gt: *mut LuaTable,
  pub openupval: *mut UpVal,
  pub gclist: *mut GcObject,
  pub namecall: *mut tstring,
  pub userdata: *mut c_void,
}

pub type LuaState = lua_State;
