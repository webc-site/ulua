use alloc::boxed::Box;
use core::ptr::null_mut;

use ulua_code_gen::{
  enums::{
    include_cfg_info::IncludeCfgInfo,
    include_use_info::IncludeUseInfo,
    options::{IncludeIrPrefix, IncludeRegFlowInfo},
    target::Target,
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
      // DELIBERATE DEVIATION / 保留理由：`AssemblyOptions.annotator_context`
      // （声明在 ulua-code-gen::records::assembly_options）是交回 C ABI 的 opaque
      // 宿主 user-data——唯一消费点在 ulua-code-gen::functions::lower_impl：
      // `annotator(options.annotator_context, build.text_mut(), bytecodeid,
      // bc_location as i32)`，被调方类型 `AnnotatorFn = Option<unsafe
      // extern "C-unwind" fn(context: *mut c_void, ..)>`。字段换
      // `Option<NonNull<c_void>>` 也得在该调用点还原成同一个 `*mut c_void`；
      // 本 fixture 与 cpp `IrAssemblyTest` 一样 `annotator = None`，此值无消费方。
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
