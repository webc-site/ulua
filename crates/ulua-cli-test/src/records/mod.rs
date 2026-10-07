//! 夹具数据结构：`Completion` 对应 cpp `tests/Repl.test.cpp` 的同名 struct，
//! `ReplFixture`/`ReplWithPathFixture` 对应两个测试类（后者在前者的类型别名，
//! cpp 里两份逐行相同的实现，Rust 收敛为一处）。

pub mod completion;
pub mod repl_fixture;
pub mod repl_with_path_fixture;
