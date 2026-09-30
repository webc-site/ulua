use core::ptr::null_mut;

use crate::type_aliases::{t_value::TValue, value::Value};

/// The shared immutable nil sentinel. Reference: `VM/src/lobject.cpp:16`
/// `const TValue luaO_nilobject_ = {{NULL}, {0}, LUA_TNIL};`
///
/// `TValue` holds raw pointers so it is not `Sync`; the wrapper asserts what
/// the C++ global guarantees — the object is immutable shared data.
#[repr(transparent)]
pub struct NilSentinel(pub TValue);
/// # Safety
///
/// 哨兵是编译期常量（`LUA_O_NILOBJECT_VALUE`，经不可变 `static` 暴露，非 `static mut`），
/// 布局内无任何 `UnsafeCell`／内部可变性；其裸指针字段仅存 `null_mut()` 作为 nil 值，
/// 从不被解引用或写入。跨线程以 `&` 共享只会读到同一固定字节序列，故 `Sync` 成立。
unsafe impl Sync for NilSentinel {}

/// sentinel 的完整值，提升为 `pub const` 供 ulua-capi 导出壳复用。
pub const LUA_O_NILOBJECT_VALUE: NilSentinel = NilSentinel(TValue {
  value: Value { p: null_mut() },
  extra: [0],
  tt: 0, // LUA_TNIL
});

pub static LUA_O_NILOBJECT_: NilSentinel = LUA_O_NILOBJECT_VALUE;

/// C++ `#define LUA_O_NILOBJECT (&luaO_nilobject_)`.
pub const LUA_O_NILOBJECT: *const TValue = &LUA_O_NILOBJECT_.0 as *const TValue;
