//! 夹具层私有辅助：仅 `captured_output` / `new_fixture_state` 与 iOS 的
//! 资源目录探测（对齐 cpp `#if TARGET_OS_IPHONE`，整对模块仅 ios 编译）；
//! CLI 实现（`setupState`、`runCode`、`getCompletions` 等）
//! 一律取自 `ulua_repl_cli`，与 cpp 测试链接 CLI 的做法一致。

pub(crate) mod captured_output;
#[cfg(target_os = "ios")]
pub(crate) mod get_resource_path;
#[cfg(target_os = "ios")]
pub(crate) mod get_resource_path_0;
pub(crate) mod new_fixture_state;
