//! 夹具方法清单：cpp 两个测试类里各自定义的成员函数，按 Rust 目录化惯例
//! 一函数一文件；`ReplWithPathFixture` 与 `ReplFixture` 在 cpp 中重复实现的
//! `getCapturedOutput` 收敛到 [`repl_fixture_get_captured_output`]（前者是后者的
//! 类型别名，见 [`crate::records::repl_with_path_fixture`]）。

pub mod repl_fixture_check_completion;
pub mod repl_fixture_get_captured_output;
pub mod repl_fixture_get_completion_set;
pub mod repl_with_path_fixture_assert_output_contains_all;
pub mod repl_with_path_fixture_get_luau_directory;
pub mod repl_with_path_fixture_run_protected_require;
