use core::{
  fmt::{Debug, Formatter, Result},
  ptr,
};

use crate::{
  enums::lua_type::LuaType,
  records::{
    closure::Closure, lua_state::LuaState, lua_table::LuaTable, luau_buffer::LuaBuffer,
    t_string::TString, udata::Udata,
  },
  tagged_slot_accessors,
  type_aliases::value::Value,
};
#[derive(Clone, Copy)]
#[repr(C)]
#[derive(Default)]
pub struct TKey {
  pub value: Value,
  pub extra: [i32; 1],
  /// C++ bitfields `unsigned tt : 4; int next : 28;` packed into one 4-byte
  /// word (low 4 bits = tt, high 28 bits = signed next). Keeping them as two
  /// separate `i32`s made TKey 24 bytes (LuaNode 40) instead of 16/32, which
  /// broke the JIT ABI guard. Access via tt()/next()/set_tt()/set_next() only.
  pub tt_next: u32,
}

impl TKey {
  #[inline]
  pub fn tt(&self) -> i32 {
    (self.tt_next & 0xF) as i32
  }

  /// 判断是否为 nil 类型（cpp `ttisnil`，lobject.h:103）。
  #[inline]
  pub fn is_nil(&self) -> bool {
    self.tt() == LuaType::Nil as i32
  }

  /// 判断是否为 boolean 类型（cpp `ttisboolean`，lobject.h:104）。
  #[inline]
  pub fn is_boolean(&self) -> bool {
    self.tt() == LuaType::Boolean as i32
  }

  /// 判断是否为 number 类型（cpp `ttisnumber`，lobject.h:105）。
  #[inline]
  pub fn is_number(&self) -> bool {
    self.tt() == LuaType::Number as i32
  }

  /// 判断是否为 integer 类型（本 fork 扩展类型，`ttisinteger`）。
  #[inline]
  pub fn is_integer(&self) -> bool {
    self.tt() == LuaType::Integer as i32
  }

  /// 判断是否为 string 类型（cpp `ttisstring`，lobject.h:106）。
  #[inline]
  pub fn is_string(&self) -> bool {
    self.tt() == LuaType::String as i32
  }

  /// 判断是否为 table 类型（cpp `ttistable`，lobject.h:107）。
  #[inline]
  pub fn is_table(&self) -> bool {
    self.tt() == LuaType::Table as i32
  }

  /// 判断是否为 function 类型（cpp `ttisfunction`，lobject.h:108）。
  #[inline]
  pub fn is_function(&self) -> bool {
    self.tt() == LuaType::Function as i32
  }

  /// 判断是否为 thread 类型（cpp `ttisthread`，lobject.h:109）。
  #[inline]
  pub fn is_thread(&self) -> bool {
    self.tt() == LuaType::Thread as i32
  }

  /// 判断是否为 userdata 类型（cpp `ttisuserdata`，lobject.h:110）。
  #[inline]
  pub fn is_userdata(&self) -> bool {
    self.tt() == LuaType::UserData as i32
  }

  /// 判断是否为 lightuserdata 类型（cpp `ttislightuserdata`，lobject.h:111）。
  #[inline]
  pub fn is_lightuserdata(&self) -> bool {
    self.tt() == LuaType::LightUserData as i32
  }

  /// 判断是否为 vector 类型（cpp `ttisvector`，lobject.h:112）。
  #[inline]
  pub fn is_vector(&self) -> bool {
    self.tt() == LuaType::Vector as i32
  }

  /// 判断是否为 buffer 类型（cpp `ttisbuffer`，lobject.h:113）。
  #[inline]
  pub fn is_buffer(&self) -> bool {
    self.tt() == LuaType::Buffer as i32
  }

  // GC 引用三件套与标量读取族：与 `lua_TValue` 逐字同形的双镜像收进单源宏
  // `tagged_slot_accessors!`（宏体见 `macros/tagged_slot_accessors.rs`），签名、
  // debug_assert、body 与手写版逐点等价；`as_vector`/`as_vector_ref`（跨
  // value+extra 裸写形状）不在本宏范围，继续手写于下方。
  tagged_slot_accessors! {
    gc as_table, as_table_mut, as_table_ptr, LuaTable, *mut LuaTable, h, is_table,
      "借用为只读 Table 引用（cpp `hvalue`）。",
      "借用为可变 Table 引用。",
      "获取 Table 的裸指针。";
    gc as_closure, as_closure_mut, as_closure_ptr, Closure, *mut Closure, cl, is_function,
      "借用为只读 Closure 引用（cpp `clvalue`）。",
      "借用为可变 Closure 引用。",
      "获取 Closure 的裸指针。";
    gc_ro as_string, as_string_ptr, TString, *mut TString, ts, is_string,
      "借用为只读 TString 引用（cpp `tsvalue`）。",
      "获取 TString 的裸指针。";
    gc_cptr as_userdata, as_userdata_mut, as_userdata_ptr, Udata, *const Udata, u,
      is_userdata,
      "借用为只读 Udata 引用（cpp `uvalue`）。",
      "借用为可变 Udata 引用。",
      "获取 Udata 的原始指针（`*const Udata`）。";
    gc as_thread, as_thread_mut, as_thread_ptr, LuaState, *mut LuaState, th, is_thread,
      "借用为只读 LuaState（Thread）引用（cpp `thvalue`）。",
      "借用为可变 LuaState 引用。",
      "获取 LuaState 的原始指针（`*mut LuaState`）。";
    gc as_buffer, as_buffer_mut, as_buffer_ptr, LuaBuffer, *mut LuaBuffer, buf, is_buffer,
      "借用为只读 LuaBuffer 引用（cpp `bufvalue`）。",
      "借用为可变 LuaBuffer 引用。",
      "获取 LuaBuffer 的原始指针（`*mut LuaBuffer`）。";
    scalar_bool as_boolean, b, is_boolean,
      "读取布尔值（cpp `bvalue`）。";
    scalar as_boolean_raw, i32, b, is_boolean,
      "读取布尔原始整型（cpp `bvalue` 原样 i32，true 必为 1）。";
    scalar as_number, f64, n, is_number,
      "读取浮点数值（cpp `nvalue`）。";
    scalar as_integer, i64, l, is_integer,
      "读取 64 位整型值（`lvalue`）。";
  }

  /// 读取 3 分量向量（cpp `vvalue`；`value + extra` 连续 12 字节）。
  #[inline]
  pub fn as_vector(&self) -> [f32; 3] {
    *self.as_vector_ref()
  }

  /// 借用为 3 分量向量切片引用。
  #[inline]
  pub fn as_vector_ref(&self) -> &[f32; 3] {
    debug_assert!(self.is_vector());
    unsafe { &*(ptr::from_ref(self).cast::<[f32; 3]>()) }
  }

  #[inline]
  pub fn set_tt(&mut self, tt: i32) {
    self.tt_next = (self.tt_next & !0xF) | ((tt as u32) & 0xF);
  }

  #[inline]
  pub fn next(&self) -> i32 {
    // `int next : 28` — sign-extend from the 28-bit field (bits 4..31).
    (self.tt_next as i32) >> 4
  }

  #[inline]
  pub fn set_next(&mut self, next: i32) {
    self.tt_next = (self.tt_next & 0xF) | ((next as u32) << 4);
  }
}

impl Debug for TKey {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("TKey")
      .field("extra", &self.extra)
      .field("tt", &self.tt())
      .field("next", &self.next())
      .finish_non_exhaustive()
  }
}
