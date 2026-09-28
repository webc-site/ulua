use core::{ffi::c_void, ptr::null_mut, str::from_utf8};

use super::LuaState;
use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_isnumber::lua_isnumber, lua_isstring::lua_isstring, lua_l_checkany::lua_l_checkany,
    lua_l_checkboolean::lua_l_checkboolean, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_checknumber::lua_l_checknumber, lua_l_checktype::lua_l_checktype,
    lua_l_optboolean::lua_l_optboolean, lua_objlen::lua_objlen, lua_rawequal::lua_rawequal,
    lua_toboolean::lua_toboolean, lua_tointegerx::lua_tointegerx,
    lua_tolightuserdata::lua_tolightuserdata_ref, lua_tolstring::lua_tolstring_ref,
    lua_tonumberx::lua_tonumberx, lua_tothread::lua_tothread, lua_type::lua_type,
  },
};

impl LuaState {
  #[inline(always)]
  pub fn type_of(&self, idx: i32) -> LuaType {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方
    // `# Safety` 其余前提（`idx` 为合法（伪）索引）由调用方按文档保证。
    let t = unsafe { lua_type(self.read_ptr(), idx) };
    LuaType::from_c_int(t).unwrap_or(LuaType::None)
  }

  #[inline(always)]
  pub fn is_nil(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::Nil
  }

  #[inline(always)]
  pub fn is_none_or_nil(&self, idx: i32) -> bool {
    matches!(self.type_of(idx), LuaType::None | LuaType::Nil)
  }

  #[inline(always)]
  pub fn is_boolean(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::Boolean
  }

  #[inline(always)]
  pub fn is_number(&self, idx: i32) -> bool {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_isnumber(self.read_ptr(), idx) != 0 }
  }

  #[inline(always)]
  pub fn is_string(&self, idx: i32) -> bool {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_isstring(self.read_ptr(), idx) != 0 }
  }

  #[inline(always)]
  pub fn is_table(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::Table
  }

  #[inline(always)]
  pub fn is_function(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::Function
  }

  #[inline(always)]
  pub fn is_thread(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::Thread
  }

  #[inline(always)]
  pub fn is_buffer(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::Buffer
  }

  #[inline(always)]
  pub fn is_object(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::Object
  }

  #[inline(always)]
  pub fn is_integer_64(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::Integer
  }

  #[inline(always)]
  pub fn is_userdata(&self, idx: i32) -> bool {
    self.type_of(idx) == LuaType::UserData
  }

  #[inline(always)]
  pub fn to_boolean(&self, idx: i32) -> bool {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_toboolean(self.read_ptr(), idx) != 0 }
  }

  #[inline(always)]
  pub fn to_number(&self, idx: i32) -> Option<f64> {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_tonumberx(self.read_ptr(), idx) }
  }

  #[inline(always)]
  pub fn to_integer(&self, idx: i32) -> Option<i32> {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_tointegerx(self.read_ptr(), idx) }
  }

  #[inline(always)]
  pub fn obj_len(&self, idx: i32) -> usize {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_objlen(self.read_ptr(), idx) as usize }
  }

  #[inline(always)]
  pub fn to_bytes<'a>(&mut self, idx: i32) -> Option<&'a [u8]> {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_tolstring_ref(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn raw_equal(&self, idx1: i32, idx2: i32) -> bool {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_rawequal(self.read_ptr(), idx1, idx2) != 0 }
  }

  #[inline(always)]
  pub fn to_str<'a>(&mut self, idx: i32) -> Option<&'a str> {
    self.to_bytes(idx).and_then(|b| from_utf8(b).ok())
  }

  #[inline(always)]
  pub fn check_bytes<'a>(&mut self, idx: i32) -> &'a [u8] {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_checklstring_ref(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn check_str<'a>(&mut self, idx: i32) -> &'a str {
    let b = self.check_bytes(idx);
    from_utf8(b).unwrap_or("")
  }

  #[inline(always)]
  pub fn opt_bytes<'a>(&mut self, idx: i32, def: &'a [u8]) -> &'a [u8] {
    if self.is_none_or_nil(idx) {
      def
    } else {
      self.check_bytes(idx)
    }
  }

  #[inline(always)]
  pub fn check_integer(&mut self, narg: i32) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_checkinteger(self.as_mut_ptr(), narg) }
  }

  #[inline(always)]
  pub fn check_number(&mut self, narg: i32) -> f64 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_checknumber(self.as_mut_ptr(), narg) }
  }

  #[inline(always)]
  pub fn check_type(&mut self, narg: i32, t: LuaType) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_checktype(self.as_mut_ptr(), narg, t as i32) }
  }

  #[inline(always)]
  pub fn check_any(&mut self, narg: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_checkany(self.as_mut_ptr(), narg) }
  }

  #[inline(always)]
  pub fn check_boolean(&mut self, narg: i32) -> bool {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_checkboolean(self.as_mut_ptr(), narg) != 0 }
  }

  #[inline(always)]
  pub fn check_integer_64(&mut self, narg: i32) -> i64 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_checkinteger_64(self.as_mut_ptr(), narg) }
  }

  /// 仅本模块的 `to_lightuserdata_ptr`（全仓唯一消费面）使用；外部一律走 `_ptr` 变体。
  #[inline(always)]
  pub(super) fn to_lightuserdata(&self, idx: i32) -> Option<*mut c_void> {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_tolightuserdata_ref(self.read_ptr(), idx) }
  }

  #[inline(always)]
  pub fn to_lightuserdata_ptr(&self, idx: i32) -> *mut c_void {
    self.to_lightuserdata(idx).unwrap_or(null_mut())
  }

  /// 仅本模块的 `to_thread_ptr`（全仓唯一消费面）使用；外部一律走 `_ptr` 变体。
  #[inline(always)]
  pub(super) fn to_thread(&self, idx: i32) -> Option<*mut LuaState> {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_tothread(self.read_ptr(), idx) }
  }

  #[inline(always)]
  pub fn to_thread_ptr(&self, idx: i32) -> *mut LuaState {
    self.to_thread(idx).unwrap_or(null_mut())
  }

  #[inline(always)]
  pub fn opt_boolean(&mut self, narg: i32, def: bool) -> bool {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_optboolean(self.as_mut_ptr(), narg, def) }
  }
}
