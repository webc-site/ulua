//! NUL 结尾字节串字面量（`b"name\0"`）→ `*const c_char` 的测试侧出口。
//!
//! review.md §10：C 字符串字面量（c 前缀）仅豁免于 C ABI 实现层（ulua-vm/ulua-capi
//! 的 `lua_*.rs`/`*_shim.rs`），测试代码一律用 `b"..\0"` 字节串，并只在调用
//! `*const c_char` 契约 API 的收口点转成裸指针。写入方向的实现不再本地复制，
//! 直接复用 ulua-common `c_str` 门面的 [`cstr`]（与读取方向的 [`cstr_cow`] /
//! [`cstr_bytes`] 对偶），本模块仅作用例侧语义化重导出，不散落 `.as_ptr().cast()`。
//!
//! [`cstr`]: ulua_common::functions::c_str::cstr
//! [`cstr_cow`]: ulua_common::functions::c_str::cstr_cow
//! [`cstr_bytes`]: ulua_common::functions::c_str::cstr_bytes

pub use ulua_common::functions::c_str::cstr;
