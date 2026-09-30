use ulua_cli_lib::functions::copts::copts_with;
use ulua_compiler::records::compile_options::CompileOptions;

use crate::records::global_options::with;

/// cpp `copts()`: 以本 CLI 的线程局部全局选项构造 `CompileOptions`。
/// 结构组装已上收到 `ulua-cli-lib::functions::copts::copts_with`（与 bytecode
/// 等 CLI 共用同一份字段映射），此处只做「全局读数 → 参数」适配。
/// 并行编译下每个任务先经 `global_options::restore` 在本线程重放快照再调用。
pub fn copts() -> CompileOptions {
  with(|opts| {
    copts_with(
      opts.optimization_level,
      opts.debug_level,
      opts.type_info_level,
      opts.vector_lib.as_deref(),
      opts.vector_ctor.as_deref(),
      opts.vector_type.as_deref(),
    )
  })
}
