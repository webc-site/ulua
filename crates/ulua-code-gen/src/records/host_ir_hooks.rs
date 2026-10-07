//! 译自 CodeGenOptions.h:74（逐字段对应）。
//! Source: `CodeGen/include/Luau/CodeGenOptions.h`

use crate::type_aliases::api::{
  HostUserdataAccessHandler, HostUserdataMetamethodBytecodeType, HostUserdataMetamethodHandler,
  HostUserdataNamecallHandler, HostUserdataOperationBytecodeType, HostVectorAccessHandler,
  HostVectorNamecallHandler, HostVectorOperationBytecodeType,
};

#[derive(Debug, Clone, Default)]
pub struct HostIrHooks {
  pub vector_access_bytecode_type: HostVectorOperationBytecodeType,
  pub vector_namecall_bytecode_type: HostVectorOperationBytecodeType,
  pub vector_access: HostVectorAccessHandler,
  pub vector_namecall: HostVectorNamecallHandler,
  pub userdata_access_bytecode_type: HostUserdataOperationBytecodeType,
  pub userdata_metamethod_bytecode_type: HostUserdataMetamethodBytecodeType,
  pub userdata_namecall_bytecode_type: HostUserdataOperationBytecodeType,
  pub userdata_access: HostUserdataAccessHandler,
  pub userdata_metamethod: HostUserdataMetamethodHandler,
  pub userdata_namecall: HostUserdataNamecallHandler,
}
