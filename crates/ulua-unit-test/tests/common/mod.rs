// ulua-unit-test 集成测试共享样板目录。
// 约定：一文件一职责，子模块在此经 mod.rs 声明；各 tests/*.rs 以 `mod common;` 引入，
// 其中的 `#[macro_export]` 宏即提升到该集成测试 crate 根按裸名调用。
// 新增共享前奏/助手时按职责另建子文件并在本文件追加声明。

// 类型推断/前端测试的 fixture 前奏宏（fx_check!/bs_check!）。
mod type_check;
