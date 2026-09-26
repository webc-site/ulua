//! 测试夹具共用的 C 名字常量（cpp Compiler.test.cpp/IrLowering.test.cpp 的
//! `vector`/`Vector3`/`new`/`test` 字面量）。
//!
//! §10 双形态：NUL 结尾 `&[u8]` 形态面向 `*const c_char` 契约的喂指针调用点
//! （`.as_ptr().cast()`），`*_STR` 形态为 `&str` 逻辑视图（取长度/内容）——
//! 不引入 `CStr` 类型。

/// 库名 `test`（NUL 结尾形态，供 `*const c_char` 数组）。
pub const NAME_TEST: &[u8] = b"test\0";
/// 库名 `test`（`&str` 形态，ptr+len 调用点免魔法长度）。
pub const NAME_TEST_STR: &str = "test";

/// 库名 `vector`（NUL 结尾形态，供 `*const c_char` 数组）。
pub const NAME_VECTOR: &[u8] = b"vector\0";

/// 类型/库名 `Vector3`（NUL 结尾形态，供 `*const c_char` 数组）。
pub const NAME_VECTOR3: &[u8] = b"Vector3\0";

/// `Vector3` 的构造器成员名 `new`（NUL 结尾形态，供 `*const c_char` 数组）。
pub const NAME_NEW: &[u8] = b"new\0";
