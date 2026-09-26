//! GC 枚举边名 "metatable"（cpp lgcdebug 元表边）。
//!
//! NUL 结尾字节串面向 `*const c_char` 契约——不引入 `CStr` 类型。

/// NUL 结尾边名（`enumedge`/`enumedges` 的 `*const c_char` 契约用）。
pub const EDGE_METATABLE: &[u8] = b"metatable\0";
