use std::{any::Any, panic::resume_unwind};

use ulua_compiler::records::compile_error::CompileError;

use crate::functions::report_compile_error::report_compile_error_string;

/// 编译管线的 `catch_unwind` 失败面（bytecode/compile 两 CLI 同构样板收口）：
/// `CompileError` 负载上报后折算 `None`（调用方按编译失败止步），其余负载
/// 原样 `resume_unwind`——真实缺陷仍炸出原 panic，不被吞掉。
///
/// 保留 `Box<dyn Any + Send>`：形参类型即 `catch_unwind` 返回的错误形态
/// （负载类型集合运行期开放，由 std API 定死），非可选分发位点。
pub fn report_compile_panic<T>(name: &str, result: Result<T, Box<dyn Any + Send>>) -> Option<T> {
  match report_compile_panic_string(name, result) {
    Ok(value) => Some(value),
    Err(message) => {
      eprint!("{message}");
      None
    }
  }
}

/// [`report_compile_panic`] 的无副作用版本：`CompileError` 折算为
/// `Err(消息)`（调用方自行缓冲输出），其余负载仍原样 `resume_unwind`。
/// 供并行 compile CLI 按文件收集 stderr、主线程按原顺序统一打印。
/// 入参的 `dyn` 保留理由同 [`report_compile_panic`]（同一枚 `catch_unwind` 载荷）。
pub fn report_compile_panic_string<T>(
  name: &str,
  result: Result<T, Box<dyn Any + Send>>,
) -> Result<T, String> {
  match result {
    Ok(value) => Ok(value),
    Err(payload) => {
      if let Some(error) = payload.downcast_ref::<CompileError>() {
        Err(report_compile_error_string(name, error))
      } else {
        resume_unwind(payload)
      }
    }
  }
}
