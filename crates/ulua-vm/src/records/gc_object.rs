use core::{mem::ManuallyDrop, ptr::addr_of_mut};

pub use crate::records::t_string::TString;
use crate::{
  enums::lua_type::LuaType,
  gc_object_accessors,
  records::{
    closure::Closure, g_cheader::GCheader, lua_state::LuaState, lua_table::LuaTable,
    luau_buffer::LuauBuffer, luau_class::LuauClass, luau_object::LuauObject, proto::Proto,
    t_string::tstring, udata::Udata, up_val::UpVal,
  },
};

/// GC 堆中所有对象的统一底层存储联合体
#[repr(C)]
pub union GcObject {
  pub gch: GCheader,
  pub ts: ManuallyDrop<tstring>,
  pub u: ManuallyDrop<Udata>,
  pub cl: ManuallyDrop<Closure>,
  pub h: ManuallyDrop<LuaTable>,
  pub p: ManuallyDrop<Proto>,
  pub uv: ManuallyDrop<UpVal>,
  pub th: ManuallyDrop<LuaState>,
  pub buf: ManuallyDrop<LuauBuffer>,
  pub lclass: ManuallyDrop<LuauClass>,
  pub lobject: ManuallyDrop<LuauObject>,
}

pub type GCObject = GcObject;

/// GC 对象的只读借用枚举视图，按其类型标签安全解构
#[derive(Debug)]
pub enum GcView<'a> {
  String(&'a TString),
  Table(&'a LuaTable),
  Closure(&'a Closure),
  UserData(&'a Udata),
  Thread(&'a LuaState),
  Buffer(&'a LuauBuffer),
  Class(&'a LuauClass),
  Object(&'a LuauObject),
  Proto(&'a Proto),
  UpVal(&'a UpVal),
}

/// GC 对象的可变借用枚举视图，按其类型标签安全解构
#[derive(Debug)]
pub enum GcViewMut<'a> {
  String(&'a mut TString),
  Table(&'a mut LuaTable),
  Closure(&'a mut Closure),
  UserData(&'a mut Udata),
  Thread(&'a mut LuaState),
  Buffer(&'a mut LuauBuffer),
  Class(&'a mut LuauClass),
  Object(&'a mut LuauObject),
  Proto(&'a mut Proto),
  UpVal(&'a mut UpVal),
}

impl GcObject {
  /// 获取只读通用 GC 头
  #[inline]
  pub fn header(&self) -> &GCheader {
    unsafe { &self.gch }
  }

  /// 获取可变通用 GC 头
  #[inline]
  pub fn header_mut(&mut self) -> &mut GCheader {
    unsafe { &mut self.gch }
  }

  /// 获取类型 tag 字节值
  #[inline]
  pub fn tt(&self) -> u8 {
    self.header().tt
  }

  /// 获取 marked 标记位
  #[inline]
  pub fn marked(&self) -> u8 {
    self.header().marked
  }

  /// 获取内存类别
  #[inline]
  pub fn memcat(&self) -> u8 {
    self.header().memcat
  }

  /// 获取类型枚举判别式
  #[inline]
  pub fn lua_type(&self) -> Option<LuaType> {
    self.header().lua_type()
  }
}

// 10 个 GC 类型 × 5 种同构壳（as_X / as_X_mut / try_as_X / try_as_X_ref /
// try_as_X_ptr）与 as_view/as_view_mut、gclist/set_gclist 的单源生成：
// 语义与手写版逐点等价（union 读前先匹配 tag；`try_as_*<'a>` 的 `'a` 由调用方
// 指定），宏体见 `macros/gc_object_accessors.rs`。
gc_object_accessors! {
  as_table, as_table_mut, try_as_table, try_as_table_ref, try_as_table_ptr,
  "表", LuaTable, h, is_table, LuaType::Table, Table;
  as_closure, as_closure_mut, try_as_closure, try_as_closure_ref, try_as_closure_ptr,
  "闭包", Closure, cl, is_closure, LuaType::Function, Closure;
  as_proto, as_proto_mut, try_as_proto, try_as_proto_ref, try_as_proto_ptr,
  " Proto ", Proto, p, is_proto, LuaType::Proto, Proto;
  as_string, as_string_mut, try_as_string, try_as_string_ref, try_as_string_ptr,
  "字符串", TString, ts, is_string, LuaType::String, String;
  as_udata, as_udata_mut, try_as_udata, try_as_udata_ref, try_as_udata_ptr,
  " UserData ", Udata, u, is_userdata, LuaType::UserData, UserData;
  as_thread, as_thread_mut, try_as_thread, try_as_thread_ref, try_as_thread_ptr,
  "线程（LuaState）", LuaState, th, is_thread, LuaType::Thread, Thread;
  as_buffer, as_buffer_mut, try_as_buffer, try_as_buffer_ref, try_as_buffer_ptr,
  " Buffer ", LuauBuffer, buf, is_buffer, LuaType::Buffer, Buffer;
  as_class, as_class_mut, try_as_class, try_as_class_ref, try_as_class_ptr,
  " Class ", LuauClass, lclass, is_class, LuaType::Class, Class;
  as_object, as_object_mut, try_as_object, try_as_object_ref, try_as_object_ptr,
  " Object ", LuauObject, lobject, is_object, LuaType::Object, Object;
  as_upval, as_upval_mut, try_as_upval, try_as_upval_ref, try_as_upval_ptr,
  " UpVal ", UpVal, uv, is_upval, LuaType::Upval, UpVal;
  =>
  LuaType::Table => h,
  LuaType::Function => cl,
  LuaType::Thread => th,
  LuaType::Proto => p,
  LuaType::Class => lclass,
  LuaType::Object => lobject,
}

/// 若裸指针非空，安全尝试获取其只读视图
///
/// # Safety
/// 若 `gco` 非空，必须指向有效存活的 `GcObject` 内存。
#[inline]
pub unsafe fn try_as_view<'a>(gco: *const GcObject) -> Option<GcView<'a>> {
  if gco.is_null() {
    None
  } else {
    unsafe { (*gco).as_view() }
  }
}

/// 若裸指针非空，安全尝试获取其可变视图
///
/// # Safety
/// 若 `gco` 非空，必须指向有效存活的 `GcObject` 内存，且生命周期 `'a` 内不发生别名冲突。
#[inline]
pub unsafe fn try_as_view_mut<'a>(gco: *mut GcObject) -> Option<GcViewMut<'a>> {
  if gco.is_null() {
    None
  } else {
    unsafe { (*gco).as_view_mut() }
  }
}
