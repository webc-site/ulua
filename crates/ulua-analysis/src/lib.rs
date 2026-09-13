// crate 级 allow:not_unsafe_ptr_arg_deref
//
// 本 crate 是 Luau C API 的 Rust 封装层，公共函数大量以 `*mut LuaState` 等
// 裸指针为参数，并在函数体内通过 unsafe 解引用。这些函数语义上由 C API 的
// FFI 契约保证安全（指针有效性、生命周期由调用方维护），逐个标注 unsafe
// 会导致 2000+ 处调用方连锁改动，故在 crate 级统一豁免该 lint。

extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;
pub mod type_aliases;

extern crate self as ulua_analysis;

pub use ulua_ast::rtti;
pub use ulua_common::{FFlag, FInt};
