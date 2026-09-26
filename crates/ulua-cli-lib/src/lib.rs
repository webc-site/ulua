extern crate alloc;

pub mod enums;
pub mod functions;
pub mod methods;
pub mod records;

/// CLI 集成测试共享脚手架；默认不编译，测试侧通过 `test-utils` feature 启用。
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
