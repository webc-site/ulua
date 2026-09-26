//! 数据符号导出壳（源：ulua-vm/src/macros/lua_o_nilobject.rs）。
//! 注意：导出的是独立副本——C 侧地址与 ulua-vm 内部 static 地址不同
//! （当前无任何消费者，可接受）；vm 内部指针同一性不受影响。

use ulua_vm::macros::lua_o_nilobject::{LUA_O_NILOBJECT_VALUE, NilSentinel};

#[unsafe(export_name = "ulua_luaO_nilobject_")]
pub static LUA_O_NILOBJECT_: NilSentinel = LUA_O_NILOBJECT_VALUE;
