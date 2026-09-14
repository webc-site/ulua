//! Translated from CodeGenOptions.h:74 (field-for-field).
//! Node: `cxx:Record:Luau.CodeGen:CodeGen/include/Luau/CodeGenOptions.h:74:host_ir_hooks`
//! Source: `CodeGen/include/Luau/CodeGenOptions.h`
//! Graph edges:
//! - declared_by: source_file CodeGen/include/Luau/CodeGenOptions.h
//! - incoming:
//!   - declares <- source_file CodeGen/include/Luau/CodeGenOptions.h
//!   - type_ref <- record CompilationOptions (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref <- record IrBuilder (CodeGen/include/Luau/IrBuilder.h)
//!   - type_ref <- method IrBuilder::IrBuilder (CodeGen/src/IrBuilder.cpp)
//!   - type_ref <- function analyzeBytecodeTypes (CodeGen/src/BytecodeAnalysis.cpp)
//!   - type_ref <- record IrAssemblyFixture (tests/IrAssembly.test.cpp)
//!   - type_ref <- record IrBuilderFixture (tests/IrBuilder.test.cpp)
//! - outgoing:
//!   - type_ref -> type_alias HostVectorOperationBytecodeType (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> type_alias HostVectorAccessHandler (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> type_alias HostVectorNamecallHandler (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> type_alias HostUserdataOperationBytecodeType (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> type_alias HostUserdataMetamethodBytecodeType (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> type_alias HostUserdataAccessHandler (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> type_alias HostUserdataMetamethodHandler (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> type_alias HostUserdataNamecallHandler (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item HostIrHooks

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
