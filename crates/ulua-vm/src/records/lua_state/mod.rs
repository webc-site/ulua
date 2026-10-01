use core::ffi::c_void;

use crate::{
  records::{
    call_info::CallInfo, g_cheader::GCheader, gc_object::GcObject, global_state::global_State,
    lua_table::LuaTable, t_string::tstring, up_val::UpVal,
  },
  type_aliases::stk_id::StkId,
};

mod access;
mod error;
mod stack;
mod table;
mod thread;

#[repr(C)]
#[derive(Debug, Default)]
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
  pub stacksize: i32,
  pub size_ci: i32,
  pub n_ccalls: u16,
  pub base_ccalls: u16,
  pub cachedslot: i32,
  pub gt: *mut LuaTable,
  pub openupval: *mut UpVal,
  pub gclist: *mut GcObject,
  pub namecall: *mut tstring,
  pub userdata: *mut c_void,
}

pub type LuaState = lua_State;

impl LuaState {
  #[inline(always)]
  pub fn as_mut_ptr(&mut self) -> *mut Self {
    self as *mut Self
  }

  #[inline(always)]
  pub fn as_ptr(&self) -> *const Self {
    self as *const Self
  }

  /// 只读转发辅助：C-ABI 镜像函数族（`lua_type` 等 getter）的签名统一收
  /// `*mut Self`，即使被调方仅读取。本方法把 `*const → *mut` 的无写 provenance
  /// 抹除集中到这一处并附契约，替代散落的 `self.as_ptr() as *mut Self` 裸 cast。
  ///
  /// 使用契约：返回值仅可传给按其 `# Safety` 文档**不写穿**该指针、不将其逸出
  /// 保存的被调方；违约即别名 UB。
  #[inline(always)]
  pub(crate) fn read_ptr(&self) -> *mut Self {
    self as *const Self as *mut Self
  }
}
