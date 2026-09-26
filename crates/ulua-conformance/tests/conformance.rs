#[path = "common/mod_conf.rs"]
mod common;

extern crate alloc;

// 移植自 `cpp/tests/Conformance.test.cpp` 的 conformance 用例集。
// 按 fixture 领域分组，一组一个文件，本文件只做 mod 声明。
#[path = "conformance/api.rs"]
/// C API 直调用例（上游 TEST_CASE("Api*")）
mod api;
#[path = "conformance/bytecode.rs"]
/// 字节码产物与编译规模上限用例
mod bytecode;
#[path = "conformance/codegen.rs"]
/// 原生 codegen / IR 用例（上游 TEST_CASE("Native*")、"Codegen*")）
mod codegen;
#[path = "conformance/coroutine.rs"]
/// 协程 / yield 续跑用例
mod coroutine;
#[path = "conformance/debugger.rs"]
/// 调试器、断点、中断与 coverage 钩子用例
mod debugger;
#[path = "conformance/direct_field_access.rs"]
/// userdata 直接字段访问 handler 用例
mod direct_field_access;
#[path = "conformance/environment.rs"]
/// _G 沙箱与去库环境用例
mod environment;
#[path = "conformance/errors.rs"]
/// 错误对象与 protected call 语义用例
mod errors;
#[path = "conformance/feedback_vector.rs"]
/// feedback vector（sealed / inline / namecall）用例
mod feedback_vector;
#[path = "conformance/gc.rs"]
/// GC、引用与分配失败用例
mod gc;
#[path = "conformance/language.rs"]
/// 语言基础特性 fixture 用例
mod language;
#[path = "conformance/math.rs"]
/// 数值与日期标准库 fixture 用例
mod math;
#[path = "conformance/shared_code_allocator.rs"]
/// native code allocator 模块/原型引用计数用例
mod shared_code_allocator;
#[path = "conformance/strings.rs"]
/// 字符串、模式匹配、UTF-8 与 buffer 标准库用例
mod strings;
#[path = "conformance/tables.rs"]
/// table / 排序 / 变参 / 函数调用 fixture 用例
mod tables;
#[path = "conformance/types.rs"]
/// 类型标注、RTTI 与 class 语法用例
mod types;
#[path = "conformance/userdata.rs"]
/// userdata 标签、对齐与元表用例
mod userdata;
#[path = "conformance/vector.rs"]
/// vector 库与 vector 常量用例
mod vector;
