use core::ptr::null_mut;

use crate::{
  records::{lua_t_value::TValue, t_key::TKey},
  type_aliases::{lua_node::LuaNode, value::Value},
};

/// The shared immutable empty-hash sentinel. Reference: `VM/src/ltable.cpp:48`
/// `const LuaNode luaH_dummynode = {{{NULL},{0},LUA_TNIL}, {{NULL},{0},LUA_TNIL,0}};`
///
/// `LuaNode` holds raw pointers so it is not `Sync`; the wrapper asserts what
/// the C++ global guarantees — the object is immutable shared data.
#[repr(transparent)]
pub struct DummyNodeSentinel(pub LuaNode);
unsafe impl Sync for DummyNodeSentinel {}

#[unsafe(export_name = "ulua_luaH_dummynode")]
pub static LUA_H_DUMMYNODE: DummyNodeSentinel = DummyNodeSentinel(LuaNode {
  val: TValue {
    value: Value { p: null_mut() },
    extra: [0],
    tt: 0, // LUA_TNIL
  },
  key: TKey {
    value: Value { p: null_mut() },
    extra: [0],
    tt_next: 0, // tt=LUA_TNIL, next=0 (packed)
  },
});

pub use LUA_H_DUMMYNODE as luaH_dummynode;

/// C++ `#define dummynode (&luaH_dummynode)`.
pub const DUMMYNODE: *const LuaNode = &LUA_H_DUMMYNODE.0;

pub use DUMMYNODE as dummynode;
