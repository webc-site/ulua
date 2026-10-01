use crate::records::{
  closure::Closure, gc_object::GCObject, lua_state::LuaState, lua_t_value::TValue,
  lua_table::LuaTable, luau_buffer::LuauBuffer, luau_class::LuauClass, luau_object::LuauObject,
  proto::Proto, t_key::TKey, t_string::tstring, udata::Udata, up_val::UpVal,
};
#[macro_export]
macro_rules! ttype {
  ($o:expr) => {
    $crate::macros::ttype::TTypeOf::ttype($o)
  };
}

pub use ttype;

/// tag 读取的唯一按类型扩展点：各承载对象只声明「如何从 `&self` 读自己的类型 tag」。
///
/// 此前每个（类型 × 指针/引用形态）都手写一份 `unsafe fn ttype`（26 个 impl + 2 个
/// 双份展开宏）；形态适配与按类型读 tag 两层职责混在一起。收口到本 trait 后，
/// `TTypeOf` 只剩 4 个 blanket impl，单态化在编译期选定各类型的 `raw_tag`，
/// 运行期不再有任何按类型手写的分派体。
pub trait TTag {
  /// 读取本对象的类型 tag。`TValue`/`TKey` 为 i32 域（`TKey` 经 tt() 做 4-bit 抽取），
  /// GC 对象为 u8 域（`gch.tt`/`hdr.tt`/`tt` 字段），均为非负值，统一返回 `i32`
  /// 供 `TTypeOf` 收敛为 `u32`——与收口前各 impl 的逐类型 `as u32` 结果逐位一致。
  fn raw_tag(&self) -> i32;
}

pub trait TTypeOf {
  /// # Safety
  ///
  /// If `self` is a raw pointer, it must be valid and properly aligned.
  unsafe fn ttype(self) -> u32;
}

/// # Safety
///
/// 四个 blanket impl 覆盖 `ttype!` 的全部接收者形态：指针形态（`*const T`/`*mut T`）
/// 解引用读 tag，调用时 `self` 必须非空、按 `T` 对齐且指向已初始化的 `T`；
/// 引用形态（`&T`/`&mut T`）由 Rust 类型系统保证有效性，实现不解引用裸指针，
/// `unsafe` 仅为满足 trait 形状。
impl<T: TTag> TTypeOf for *const T {
  #[inline(always)]
  unsafe fn ttype(self) -> u32 {
    unsafe { (*self).raw_tag() as u32 }
  }
}

impl<T: TTag> TTypeOf for *mut T {
  #[inline(always)]
  unsafe fn ttype(self) -> u32 {
    unsafe { (*self).raw_tag() as u32 }
  }
}

impl<T: TTag> TTypeOf for &T {
  #[inline(always)]
  unsafe fn ttype(self) -> u32 {
    self.raw_tag() as u32
  }
}

impl<T: TTag> TTypeOf for &mut T {
  #[inline(always)]
  unsafe fn ttype(self) -> u32 {
    self.raw_tag() as u32
  }
}

impl TTag for TValue {
  #[inline(always)]
  fn raw_tag(&self) -> i32 {
    self.tt()
  }
}

impl TTag for TKey {
  #[inline(always)]
  fn raw_tag(&self) -> i32 {
    self.tt()
  }
}

impl TTag for GCObject {
  #[inline(always)]
  fn raw_tag(&self) -> i32 {
    // `gch` 是 union 各变体的公共首前缀（cpp `lobject.h` GCHeader 恒在偏移 0），
    // 已初始化的 GCObject 读 `gch.tt` 与读任何变体的 header 同值
    unsafe { self.gch.tt as i32 }
  }
}

/// GCheader 前缀形态（cpp `commonlua.h` 各对象首字段 `GCHeader`）：tag 存于 `hdr.tt`。
macro_rules! impl_ttag_hdr {
  ($($t:ty),* $(,)?) => {
    $(
      impl TTag for $t {
        #[inline(always)]
        fn raw_tag(&self) -> i32 {
          self.hdr.tt as i32
        }
      }
    )*
  };
}

/// 裸 `tt: u8` 首字段形态（cpp `lobject.h` Table/Udata/Buffer/Class/Object 的 `tt`）。
macro_rules! impl_ttag_tt_field {
  ($($t:ty),* $(,)?) => {
    $(
      impl TTag for $t {
        #[inline(always)]
        fn raw_tag(&self) -> i32 {
          self.tt as i32
        }
      }
    )*
  };
}

impl_ttag_hdr!(tstring, Closure, LuaState, Proto, UpVal);
impl_ttag_tt_field!(LuaTable, Udata, LuauBuffer, LuauClass, LuauObject);
