//! C ABI 数据符号导出壳：把 ulua-vm 的 `pub const`/`const fn` 以 `#[unsafe(export_name)]`
//! 静态量形式导出给 C 侧链接。各文件头注明导出的是独立副本，C 侧地址与 vm 内部地址不同、
//! 无指针同一性依赖；消费者（当前无）按 C 符号只读使用。
pub mod dummynode;
pub mod lua_o_nilobject;
pub mod luau_f_table;
