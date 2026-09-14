use ulua_common::enums::luau_opcode::LuauOpcode;
use ulua_vm::{records::proto::Proto, type_aliases::instruction::Instruction};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
  macros::codegen_assert::CODEGEN_ASSERT,
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

static K_CODE_ENTRY_INSN: Instruction = LuauOpcode::LOP_NATIVECALL as u32;

pub fn bind_native_protos(
  module_protos: &[*mut Proto],
  native_protos: &mut [NativeProtoExecDataPtr],
  _release: bool,
) -> u32 {
  let mut protos_bound = 0u32;
  let mut proto_it = 0usize;

  for native_proto in native_protos.iter_mut() {
    let header = unsafe { &*get_native_proto_exec_data_header(native_proto.as_ptr()) };

    while proto_it != module_protos.len()
      && unsafe { (*module_protos[proto_it]).bytecodeid as u32 } != header.bytecode_id
    {
      proto_it += 1;
    }

    CODEGEN_ASSERT!(proto_it != module_protos.len());

    let proto = module_protos[proto_it];

    unsafe {
      (*proto).execdata = native_proto.as_ptr().cast();
      (*proto).exectarget = header.entry_offset_or_address as usize;
      (*proto).codeentry = &K_CODE_ENTRY_INSN;
    }

    protos_bound += 1;
  }

  protos_bound
}
