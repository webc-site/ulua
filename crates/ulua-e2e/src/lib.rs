//! `ulua-e2e` — ulua 项目的端到端 / 集成测试套件。
//!
//! 本 crate 不含生产代码：仅作为 `tests/` 下集成测试的载体，驱动六个发布
//! 二进制（`ulua`、`ulua-analyze`、`ulua-ast`、`ulua-compile`、
//! `ulua-bytecode`、`ulua-reduce`）与 `ulua` umbrella 库 API，按真实产品
//! 方式覆盖 IO、feature flags 与敌对边界用例。
