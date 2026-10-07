use alloc::vec::Vec;
use core::mem::take;

use crate::{
  functions::get_assembly_impl::AsmBuilder,
  records::{
    assembly_options::AssemblyOptions, ir_builder::IrBuilder, lowering_stats::LoweringStats,
    module::ModuleHelpers,
  },
};

/// 统一 X64/A64 的 IR→汇编输出骨架，对齐 cpp 侧 `getAssemblyFromImpl<B>` 模板：
/// 平台差异（helpers 装配、lowering 入口、指令字宽与二进制展开）全部收口进
/// [`AsmBuilder`]（见 get_assembly_impl.rs），与 `get_assembly_impl` 同构。
pub(crate) fn get_assembly_from_ir_impl<B: AsmBuilder>(
  build: &mut B,
  ir: &mut IrBuilder,
  options: AssemblyOptions,
  stats: Option<&mut LoweringStats>,
) -> Vec<u8> {
  let mut helpers = ModuleHelpers::default();
  build.assemble_helpers(&mut helpers);

  if !options.include_outlined_code && options.include_assembly {
    build.text_mut().clear();
    build.log_append(format_args!(
      "; skipping {} bytes of outlined helpers\n",
      build.get_code_size().wrapping_mul(B::CODE_UNIT)
    ));
  }

  // lower 失败原因此处不消费（cpp 同款丢弃），仅判成败；proto 传 None 由被调方判空处理
  // （cpp 同款 nullptr 实参）。
  let lowered = build
    .lower_function(ir, &mut helpers, None, options.clone(), stats)
    .is_ok();

  if !lowered && build.log_text() {
    build.log_append(format_args!("; skipping (can't lower)\n"));
  }

  if build.log_text() {
    build.log_append(format_args!("\n"));
  }

  if !build.finalize() {
    return Vec::new();
  }

  if options.output_binary {
    let mut bytes = Vec::with_capacity(build.code_len_bytes() + build.data_bytes().len());
    build.append_code_bytes(&mut bytes);
    bytes.extend_from_slice(build.data_bytes());
    bytes
  } else {
    // builder 随本函数返回即 drop：take 免大字符串 clone
    take(build.text_mut()).into_bytes()
  }
}
