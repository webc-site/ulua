use alloc::vec::Vec;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::{bytecode_builder::BytecodeBuilder, bytecode_encoder::Encoder};
use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  functions::{
    compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
    parse_pinned::parse_pinned,
  },
  records::{compile_error::CompileError, compile_options::CompileOptions},
};

/// 对应 cpp `compile(source, options, parseOptions, BytecodeEncoder* encoder)`。
/// 编码器按值接收、以 [`Encoder`] 内联静态分发（传 `NoopEncoder` 即 cpp 的空变换
/// 编码器, `into()` 零堆分配）, 不再有 `Box<dyn>` 降级。
pub fn compile<B>(
  source: &B,
  options: &CompileOptions,
  parse_options: &ParseOptions,
  encoder: impl Into<Encoder>,
) -> Vec<u8>
where
  B: AsRef<[u8]> + ?Sized,
{
  LUAU_TIMETRACE_SCOPE!("compile", "Compiler");

  let (_allocator, mut names, result) = parse_pinned(source, parse_options);

  if !result.errors.is_empty() {
    let parse_error = &result.errors[0];
    let error = format!(
      ":{}: {}",
      parse_error.get_location().begin.line + 1,
      parse_error.what()
    );
    return BytecodeBuilder::get_error(&error);
  }

  match catch_unwind(AssertUnwindSafe(|| {
    let mut bcb = BytecodeBuilder::new(Some(encoder.into()));
    compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
      &mut bcb, &result, &mut names, options,
    );
    bcb.get_bytecode().to_vec()
  })) {
    Ok(bytecode) => bytecode,
    Err(payload) => {
      // 对应 C++ `catch (CompileError& e)`：panic payload 就是被 throw 的
      // CompileError；直接取回其 location/message，而不是重新 raise 一个新错误。
      // 其他 panic 对应 cpp 中未被捕获的异常 —— 让栈展开继续向上传播。
      match payload.downcast::<CompileError>() {
        Ok(compile_error) => {
          let compile_error_location = &compile_error.location;
          let error = format!(
            ":{}: {}",
            compile_error_location.begin.line + 1,
            // 同 crate 内直接取 String 字段，免去 CStr 重建的 unsafe + 逐字节转换
            compile_error.message
          );
          BytecodeBuilder::get_error(&error)
        }
        Err(payload) => resume_unwind(payload),
      }
    }
  }
}
