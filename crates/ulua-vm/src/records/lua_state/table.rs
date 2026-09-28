use core::ffi::c_void;

use super::LuaState;
use crate::{
  functions::{
    lua_createtable::lua_createtable, lua_getfield::lua_getfield_bytes,
    lua_getmetatable::lua_getmetatable, lua_gettable::lua_gettable,
    lua_l_getmetafield::lua_l_getmetafield, lua_next::lua_next, lua_rawget::lua_rawget,
    lua_rawgeti::lua_rawgeti, lua_rawgetptagged::lua_rawgetptagged, lua_rawset::lua_rawset,
    lua_rawsetptagged::lua_rawsetptagged, lua_setfield::lua_setfield_bytes,
    lua_setmetatable::lua_setmetatable, lua_setreadonly::lua_setreadonly,
    lua_settable::lua_settable,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_registryindex::LUA_REGISTRYINDEX},
};

impl LuaState {
  #[inline(always)]
  pub fn new_table(&mut self) {
    self.create_table(0, 0);
  }

  #[inline(always)]
  pub fn create_table(&mut self, narr: i32, nrec: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_createtable(self.as_mut_ptr(), narr, nrec) }
  }

  #[inline(always)]
  pub fn get_table(&mut self, idx: i32) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_gettable(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn set_table(&mut self, idx: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_settable(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn raw_get(&mut self, idx: i32) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_rawget(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn raw_set(&mut self, idx: i32) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_rawset(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub(crate) fn raw_get_i(&mut self, idx: i32, n: i32) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_rawgeti(self.as_mut_ptr(), idx, n) }
  }

  #[inline(always)]
  pub fn set_field_bytes(&mut self, idx: i32, k: &[u8]) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_setfield_bytes(self.as_mut_ptr(), idx, k) }
  }

  #[inline(always)]
  pub fn set_field_str(&mut self, idx: i32, k: &str) {
    self.set_field_bytes(idx, k.as_bytes())
  }

  #[inline(always)]
  pub fn get_field_bytes(&mut self, idx: i32, k: &[u8]) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_getfield_bytes(self.as_mut_ptr(), idx, k) }
  }

  #[inline(always)]
  pub fn get_field_str(&mut self, idx: i32, k: &str) -> i32 {
    self.get_field_bytes(idx, k.as_bytes())
  }

  #[inline(always)]
  pub fn set_global_bytes(&mut self, name: &[u8]) {
    self.set_field_bytes(LUA_GLOBALSINDEX, name);
  }

  #[inline(always)]
  pub fn set_global_str(&mut self, name: &str) {
    self.set_global_bytes(name.as_bytes())
  }

  #[inline(always)]
  pub fn get_global_bytes(&mut self, name: &[u8]) -> i32 {
    self.get_field_bytes(LUA_GLOBALSINDEX, name)
  }

  #[inline(always)]
  pub fn get_global_str(&mut self, name: &str) -> i32 {
    self.get_global_bytes(name.as_bytes())
  }

  /// # Safety
  /// `idx` 为合法表索引，`p` 为有效指针。
  #[inline(always)]
  pub unsafe fn raw_get_ptr(&mut self, idx: i32, p: *mut c_void) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_rawgetptagged(self.as_mut_ptr(), idx, p, 0) }
  }

  /// # Safety
  /// `idx` 为合法表索引，`p` 为有效指针。
  #[inline(always)]
  pub unsafe fn raw_set_ptr(&mut self, idx: i32, p: *mut c_void) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_rawsetptagged(self.as_mut_ptr(), idx, p, 0) }
  }

  #[inline(always)]
  pub fn get_metatable_by_str(&mut self, name: &str) -> i32 {
    self.get_field_str(LUA_REGISTRYINDEX, name)
  }

  #[inline(always)]
  pub fn get_metatable_by_bytes(&mut self, name: &[u8]) -> i32 {
    self.get_field_bytes(LUA_REGISTRYINDEX, name)
  }

  #[inline(always)]
  pub fn new_metatable_by_bytes(&mut self, name: &[u8]) -> i32 {
    self.get_field_bytes(LUA_REGISTRYINDEX, name);
    if !self.is_nil(-1) {
      return 0;
    }
    self.pop(1);
    self.new_table();
    self.push_value(-1);
    self.set_field_bytes(LUA_REGISTRYINDEX, name);
    1
  }

  #[inline(always)]
  pub fn new_metatable_by_str(&mut self, name: &str) -> i32 {
    self.new_metatable_by_bytes(name.as_bytes())
  }

  #[inline(always)]
  pub fn get_ref(&mut self, ref_: i32) {
    self.raw_get_i(LUA_REGISTRYINDEX, ref_);
  }

  #[inline(always)]
  pub fn set_readonly(&mut self, idx: i32, enabled: bool) {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_setreadonly(self.as_mut_ptr(), idx, enabled as i32) }
  }

  #[inline(always)]
  pub fn set_metatable(&mut self, idx: i32) -> i32 {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_setmetatable(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn get_metatable(&mut self, idx: i32) -> bool {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_getmetatable(self.as_mut_ptr(), idx) != 0 }
  }

  #[inline(always)]
  pub fn next(&mut self, idx: i32) -> bool {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_next(self.as_mut_ptr(), idx) != 0 }
  }

  #[inline(always)]
  pub fn get_metafield_bytes(&mut self, obj: i32, event: &[u8]) -> bool {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_getmetafield(self.as_mut_ptr(), obj, event.as_ptr().cast()) != 0 }
  }
}
