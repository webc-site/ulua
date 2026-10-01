//! 数据符号导出壳（源：ulua-vm/src/macros/dummynode.rs）。
//! 注意：导出的是独立副本——C 侧地址与 ulua-vm 内部 static 地址不同
//! （当前无任何消费者，可接受）；vm 内部指针同一性不受影响。

use ulua_vm::macros::dummynode::{DummyNodeSentinel, LUA_H_DUMMYNODE_VALUE};

#[unsafe(export_name = "ulua_luaH_dummynode")]
pub static LUA_H_DUMMYNODE: DummyNodeSentinel = LUA_H_DUMMYNODE_VALUE;
