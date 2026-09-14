use core::ptr::null_mut;

use crate::type_aliases::{t_value::TValue, value::Value};

/// The shared immutable nil sentinel. Reference: `VM/src/lobject.cpp:16`
/// `const TValue luaO_nilobject_ = {{NULL}, {0}, LUA_TNIL};`
///
/// `TValue` holds raw pointers so it is not `Sync`; the wrapper asserts what
/// the C++ global guarantees — the object is immutable shared data.
#[repr(transparent)]
pub struct NilSentinel(pub TValue);
unsafe impl Sync for NilSentinel {}

#[unsafe(export_name = "ulua_luaO_nilobject_")]
pub static LUA_O_NILOBJECT_: NilSentinel = NilSentinel(TValue {
  value: Value { p: null_mut() },
  extra: [0],
  tt: 0, // LUA_TNIL
});

pub use LUA_O_NILOBJECT_ as luaO_nilobject_;

/// C++ `#define luaO_nilobject (&luaO_nilobject_)`.
pub const LUA_O_NILOBJECT: *const TValue = &LUA_O_NILOBJECT_.0 as *const TValue;
pub use LUA_O_NILOBJECT as luaO_nilobject;
