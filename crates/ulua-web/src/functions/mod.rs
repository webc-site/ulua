pub mod check_script;
pub mod execute_script;
/// cpp `Web.cpp:71` 的 `static std::string runCode`：在 cpp 是文件内静态函数，
/// 本 crate 把它留作可独立测试的单元（`tests/run_code.rs` 直接锁它的父栈平衡
/// 与错误前缀行为）。
pub mod run_code;
pub(crate) mod run_in_sandbox;
