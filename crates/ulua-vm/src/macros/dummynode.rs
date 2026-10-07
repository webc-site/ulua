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
/// 既有约定（review.md §2）：空槽结构哨兵——`Value.p = null_mut()` 作 hash 空结点的固定字节序列，从不解引用/写入，契约详见上方 `# Safety`。
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
///
/// 哨兵语义收口：`node == DUMMYNODE` 是哈希段为哨兵单格的唯一判据（cpp
/// `luaH_isdummy`），与 `lsizenode == 0` 由 `luaH_new`/`setnodevector(0)` 配对
/// 建立；`gnode!(t, 0)` 对哨兵表恒读出本单格（键/值皆 nil）。写路径从不原地
/// 写哨兵——`luaH_newkey` 以指针相等判据转 `rehash` 换发实向量（哨兵为不可变
/// `static`，向其出借 `&mut` 即别名违例）。
pub const DUMMYNODE: *const LuaNode = &LUA_H_DUMMYNODE.0;

pub use DUMMYNODE as dummynode;
