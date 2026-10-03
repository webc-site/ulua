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
    LuaType::from_c_int(lua_type(self, idx)).unwrap_or(LuaType::None)
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
    lua_isnumber(self, idx) != 0
  }

  #[inline(always)]
  pub fn is_string(&self, idx: i32) -> bool {
    lua_isstring(self, idx) != 0
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
    lua_toboolean(self, idx) != 0
  }

  #[inline(always)]
  pub fn to_number(&self, idx: i32) -> Option<f64> {
    lua_tonumberx(self, idx)
  }

  #[inline(always)]
  pub fn to_integer(&self, idx: i32) -> Option<i32> {
    lua_tointegerx(self, idx)
  }

  #[inline(always)]
  pub fn obj_len(&self, idx: i32) -> usize {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_objlen(self.read_ptr(), idx) as usize }
  }

  /// 栈槽取串字节（r16-p28 锚定形）：返回切片锚定 `&mut self` 借用——持窗期间不得
  /// 再经本 state 读参/压栈/触发分配或 GC；窗口指向栈槽串体（Lua 串不可变不移动，
  /// 槽引用钉住存活）。底层裸窗由 `lua_tolstring_ref` 的 `# Safety` 契约承载，本门面
  /// 将其借出收窄为不长于 `self` 借用。
  #[inline(always)]
  pub fn to_bytes(&mut self, idx: i32) -> Option<&[u8]> {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_tolstring_ref(self.as_mut_ptr(), idx) }
  }

  #[inline(always)]
  pub fn raw_equal(&self, idx1: i32, idx2: i32) -> bool {
    lua_rawequal(self, idx1, idx2) != 0
  }

  /// [`to_bytes`] 的 UTF-8 视图（锚定形同上）。
  #[inline(always)]
  pub fn to_str(&mut self, idx: i32) -> Option<&str> {
    self.to_bytes(idx).and_then(|b| from_utf8(b).ok())
  }

  /// 栈槽必取串字节（r16-p28 锚定形）：切片锚定 `&mut self` 借用，持窗期间不得再动
  /// state；非串实参经 `tag_error` 抛 "string expected" 发散。契约见
  /// [`lua_l_checklstring_ref`]。
  #[inline(always)]
  pub fn check_bytes(&mut self, idx: i32) -> &[u8] {
    lua_l_checklstring_ref(self, idx)
  }

  /// [`check_bytes`] 的 UTF-8 视图（锚定形同上；非法 UTF-8 回退空串，非 ASCII 串
  /// 消费方请直接用水切片）。
  #[inline(always)]
  pub fn check_str(&mut self, idx: i32) -> &str {
    let b = self.check_bytes(idx);
    from_utf8(b).unwrap_or("")
  }

  /// 可选串实参（r16-p28 锚定形）：槽为 none/nil 时回退 `def`，否则等价
  /// [`check_bytes`]。共享 `'a` 令回退串与 `self` 借用同界——两分支返回值均可读至
  /// 借用结束；持窗期间不得再动 state。
  #[inline(always)]
  pub fn opt_bytes<'a>(&'a mut self, idx: i32, def: &'a [u8]) -> &'a [u8] {
    if self.is_none_or_nil(idx) {
      def
    } else {
      self.check_bytes(idx)
    }
  }

  #[inline(always)]
  pub fn check_integer(&mut self, narg: i32) -> i32 {
    lua_l_checkinteger(self, narg)
  }

  #[inline(always)]
  pub fn check_number(&mut self, narg: i32) -> f64 {
    lua_l_checknumber(self, narg)
  }

  #[inline(always)]
  pub(crate) fn check_type(&mut self, narg: i32, t: LuaType) {
    lua_l_checktype(self, narg, t as i32)
  }

  #[inline(always)]
  pub fn check_any(&mut self, narg: i32) {
    lua_l_checkany(self, narg)
  }

  #[inline(always)]
  pub fn check_boolean(&mut self, narg: i32) -> bool {
    lua_l_checkboolean(self, narg) != 0
  }

  #[inline(always)]
  pub fn check_integer_64(&mut self, narg: i32) -> i64 {
    lua_l_checkinteger_64(self, narg)
  }

  /// 仅本模块的 `to_lightuserdata_ptr`（全仓唯一消费面）使用；外部一律走 `_ptr` 变体。
  #[inline(always)]
  pub(super) fn to_lightuserdata(&self, idx: i32) -> Option<*mut c_void> {
    // r16-v4b：被调方已转 `&LuaState` 引用形 safe fn——`&self` 即既有安全读数门面短借，
    // 原 `read_ptr()` 裸转发与 unsafe 块随消亡（无写穿面，借用窗止于本调用语句）。
    lua_tolightuserdata_ref(self, idx)
  }

  #[inline(always)]
  pub fn to_lightuserdata_ptr(&self, idx: i32) -> *mut c_void {
    self.to_lightuserdata(idx).unwrap_or(null_mut())
  }

  /// 取 `idx` 槽协程指针；非 thread 返回 `None`（可空以 `Option` 表达，不设 null 哨兵，§2）。
  #[inline(always)]
  pub(crate) fn to_thread(&self, idx: i32) -> Option<*mut LuaState> {
    // SAFETY: `self.read_ptr()` 供只读转发（见 `LuaState::read_ptr` 契约），引用重建
    // 窗止于本调用语句；被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_tothread(&mut *self.read_ptr(), idx) }
  }

  #[inline(always)]
  pub fn opt_boolean(&mut self, narg: i32, def: bool) -> bool {
    lua_l_optboolean(self, narg, def)
  }
}
