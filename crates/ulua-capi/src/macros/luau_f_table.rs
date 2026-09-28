//! 数据符号导出壳（源：ulua-vm/src/macros/luau_f_table.rs）。
//! 注意：导出的是独立副本——C 侧地址与 ulua-vm 内部 static 地址不同
//! （当前无任何消费者，可接受）；vm 内部指针同一性不受影响。

use ulua_vm::{
  macros::luau_f_table::{TABLE_LEN, build_table},
  type_aliases::luau_fast_function::LuauFastFunction,
};

#[unsafe(export_name = "ulua_luauF_table")]
pub static LUAU_F_TABLE: [LuauFastFunction; TABLE_LEN] = build_table();
