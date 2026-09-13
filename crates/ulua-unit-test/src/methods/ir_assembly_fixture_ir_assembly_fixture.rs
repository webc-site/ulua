use alloc::boxed::Box;
use core::ptr::null_mut;

use ulua_code_gen::{
  enums::{
    include_cfg_info::IncludeCfgInfo, include_ir_prefix::IncludeIrPrefix,
    include_reg_flow_info::IncludeRegFlowInfo, include_use_info::IncludeUseInfo, target::Target,
  },
  records::{
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    host_ir_hooks::HostIrHooks, ir_builder::IrBuilder,
  },
};

use crate::records::ir_assembly_fixture::IrAssemblyFixture;
impl IrAssemblyFixture {
  pub fn new() -> Self {
    let hooks = Box::new(HostIrHooks::default());
    let build = IrBuilder::ir_builder_ir_builder(&hooks);
    let options = AssemblyOptions {
      target: Target::X64Windows,
      compilation_options: CompilationOptions::default(),
      output_binary: false,
      include_assembly: true,
      include_ir: true,
      include_outlined_code: false,
      include_ir_types: true,
      include_ir_prefix: IncludeIrPrefix::No,
      include_use_info: IncludeUseInfo::No,
      include_cfg_info: IncludeCfgInfo::No,
      include_reg_flow_info: IncludeRegFlowInfo::No,
      annotator: None,
      annotator_context: null_mut(),
    };

    Self {
      hooks,
      build,
      options,
    }
  }
}

impl Default for IrAssemblyFixture {
  fn default() -> Self {
    Self::new()
  }
}
