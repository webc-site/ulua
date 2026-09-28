// native code allocator 模块/原型引用计数用例（cpp/tests/SharedCodeAllocator.test.cpp 的
// 5 个 TEST_CASE：NativeModuleRefRefcounting、NativeProtoRefcounting、NativeProtoState、
// AnonymousModuleLifetime、SharedAllocation）。
//
// 对账结论（r9 评审收口）：本模块与 `crates/ulua-unit-test/tests/shared_code_allocator.rs`
// 曾是同一 cpp 源的完整双移植。逐用例 diff：5 个用例的输入与断言口径逐条一致，且
// unit-test 版为超集（附移植形制说明与更强的助手收口注释）；两侧运行条件同为
// `luau_codegen_supported()`（纯平台谓词，与 LUAU_CODEGEN 环境变量无关），在支持平台上
// 均全量执行。本仓 conformance 运行器无按名收集/计数逻辑（`conformance.rs` 仅做 mod
// 接线，nextest 按测试目标名 `conformance` 过滤，JIT 一致性步骤亦不点名这些用例），
// 删除重复用例体不破坏收集与覆盖率。故用例体删除，行为钉唯一归属 unit-test 版；
// 本模块仅保留此对账注记。
