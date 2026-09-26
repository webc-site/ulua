//! 译自 CodeGenOptions.h:74（逐字段对应）。
//! Source: `CodeGen/include/Luau/CodeGenOptions.h`

use crate::type_aliases::{
  host_userdata_access_handler::HostUserdataAccessHandler,
  host_userdata_metamethod_bytecode_type::HostUserdataMetamethodBytecodeType,
  host_userdata_metamethod_handler::HostUserdataMetamethodHandler,
  host_userdata_namecall_handler::HostUserdataNamecallHandler,
  host_userdata_operation_bytecode_type::HostUserdataOperationBytecodeType,
  host_vector_access_handler::HostVectorAccessHandler,
  host_vector_namecall_handler::HostVectorNamecallHandler,
  host_vector_operation_bytecode_type::HostVectorOperationBytecodeType,
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
