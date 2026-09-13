use core::ffi::c_void;

use crate::{
  enums::{
    include_cfg_info::IncludeCfgInfo, include_ir_prefix::IncludeIrPrefix,
    include_reg_flow_info::IncludeRegFlowInfo, include_use_info::IncludeUseInfo, target::Target,
  },
  records::compilation_options::CompilationOptions,
  type_aliases::annotator_fn::AnnotatorFn,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AssemblyOptions {
  pub target: Target,
  pub compilation_options: CompilationOptions,
  pub output_binary: bool,
  pub include_assembly: bool,
  pub include_ir: bool,
  pub include_outlined_code: bool,
  pub include_ir_types: bool,
  pub include_ir_prefix: IncludeIrPrefix,
  pub include_use_info: IncludeUseInfo,
  pub include_cfg_info: IncludeCfgInfo,
  pub include_reg_flow_info: IncludeRegFlowInfo,
  pub annotator: AnnotatorFn,
  pub annotator_context: *mut c_void,
}
