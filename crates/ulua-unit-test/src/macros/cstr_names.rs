//! 测试夹具共用的名字常量（cpp Compiler.test.cpp/IrLowering.test.cpp 的
//! `test` 字面量）。`*const c_char` 喂指针形态已随 CompileOptions 的 Rust 化
//! 删除，仅余 `set_compile_constant_string`（ptr+len 契约）需要的 `&str` 源。

/// 常量字符串值 `test`（`set_compile_constant_string` ptr+len 调用点取
/// ptr/len 用）。
pub const NAME_TEST_STR: &str = "test";
