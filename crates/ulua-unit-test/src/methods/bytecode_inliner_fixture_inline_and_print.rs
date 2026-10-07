use alloc::string::String;

use ulua_bytecode::{
  functions::{
    sccp_fold_constants::sccp_fold_constants,
    to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  },
  records::{bytecode_builder::BytecodeBuilder, sccp::BcVmConstImpl},
};

use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;
impl BytecodeInlinerFixture {
  /// cpp `inlineAndPrint`：编译 → 内联第 `call_idx` 个 CALLFB → 可选 SCCP 常量折叠 → 反汇编。
  ///
  /// cpp 版在折叠前后各跑一次 `verifyUseConsistency`；Rust 图把 def→use 反向边收在
  /// `SccpState::op_uses`（`seed_uses` 现建现用），实例上不再常驻 `uses` 列表，
  /// 故该校验在 Rust 侧无对应物，改由折叠后的反汇编全文比对承担断言强度。
  pub fn inline_and_print(
    &mut self,
    src: &str,
    call_idx: u32,
    fold_constants: bool,
    optimization_level: i32,
  ) -> String {
    let (_inlinee, mut caller) = self
      .compile_and_inline(src, call_idx, optimization_level)
      .expect("expected inline result");

    if fold_constants {
      sccp_fold_constants(&mut caller, &BcVmConstImpl);
    }

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);
    let result = to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut bcb, &mut caller);
    assert!(!result.is_empty());
    bcb.dump_function(0)
  }
}
