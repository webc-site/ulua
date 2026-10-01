//! cpp `tests/RequireByString.test.cpp` 的 `class ReplWithPathFixture`：
//! 其构造序列、`prettyPrintSource`、`getCapturedOutput` 与
//! `tests/Repl.test.cpp` 的 `ReplFixture` 逐行相同，故此处只做类型别名，
//! 实现收敛到 [`ReplFixture`]（cpp 在两个测试文件里各复制了一份）。

use crate::records::repl_fixture::ReplFixture;

pub type ReplWithPathFixture = ReplFixture;
