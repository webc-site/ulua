//! ulua C ABI 导出层：从 ulua-vm 拆出的 `ulua_*` 符号壳。
//! 每个壳与 ulua-vm 对应实现签名一致，仅逐参数透传，零逻辑。

pub mod functions;
pub mod macros;
