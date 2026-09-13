// 本 crate 是 Luau C API / C++ AST 的 Rust 封装层，FFI 面的函数（`*mut LuaState`
// 回调、pthread/mmap extern、cpp 结构体布局）保留 C-ABI 签名；C++ 语义上由
// FFI 契约保证安全（指针有效性、生命周期由调用方维护）。

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
