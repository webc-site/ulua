use core::ptr::null_mut;

use crate::{
  records::{lua_node::LuaNode, lua_t_value::TValue, t_key::TKey},
  type_aliases::value::Value,
};

/// The shared immutable empty-hash sentinel. Reference: `VM/src/ltable.cpp:48`
/// `const LuaNode dummynode = {{{NULL},{0},LUA_TNIL}, {{NULL},{0},LUA_TNIL,0}};`
///
/// `LuaNode` holds raw pointers so it is not `Sync`; the wrapper asserts what
/// the C++ global guarantees — the object is immutable shared data.
#[repr(transparent)]
pub struct DummyNodeSentinel(pub LuaNode);
/// # Safety
///
/// 哨兵是编译期常量（`LUA_H_DUMMYNODE_VALUE`，经不可变 `static` 暴露，非 `static mut`），
/// 布局内无任何 `UnsafeCell`／内部可变性；其裸指针字段仅存 `null_mut()` 作为“空槽”值，
/// 从不被解引用或写入。跨线程以 `&` 共享只会读到同一固定字节序列，故 `Sync` 成立。
unsafe impl Sync for DummyNodeSentinel {}

/// sentinel 的完整值，提升为 `pub const` 供 ulua-capi 导出壳复用。
pub const LUA_H_DUMMYNODE_VALUE: DummyNodeSentinel = DummyNodeSentinel(LuaNode {
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

pub static LUA_H_DUMMYNODE: DummyNodeSentinel = LUA_H_DUMMYNODE_VALUE;

/// C++ `#define dummynode (&dummynode)`.
pub const DUMMYNODE: *const LuaNode = &LUA_H_DUMMYNODE.0;

pub use DUMMYNODE as dummynode;
