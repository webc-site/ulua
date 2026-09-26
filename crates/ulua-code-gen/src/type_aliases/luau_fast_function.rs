//! `LuauFastFunction` 单一来源为 ulua-vm 的
//! [`ulua_vm::type_aliases::luau_fast_function::LuauFastFunction`]，此处复导出
//! 供 JIT 消费面（`NativeContext.luau_f_table` 槽位、A64/X64 FASTCALL lowering
//! 经 `offset_of!`/`size_of!` 读取表槽位布局）使用。

pub use ulua_vm::type_aliases::luau_fast_function::LuauFastFunction;
