//! `&'static [u8]` 泄漏夹具：`invalid_utf8_roundtrip` / `graph_udata_aux_split`
//! 两个测试 binary 共用（按 binary 以 `#[path]` 单独编入，故不集中进 mod.rs，
//! 避免未用到它的 binary 报 dead_code）。字节串常量在图/常量表里以
//! `&'static [u8]` 非拥有视图存活，测试进程生命周期内无需回收。
pub fn leak_bytes(bytes: &'static [u8]) -> &'static [u8] {
  Box::leak(bytes.to_vec().into_boxed_slice())
}
