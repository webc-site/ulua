use ulua_common::enums::luau_opcode::LuauOpcode;
use ulua_vm::{records::proto::Proto, type_aliases::instruction::Instruction};

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  type_aliases::native_proto_exec_data_ptr::{
    NativeProtoExecDataHeaderExt, NativeProtoExecDataPtr,
  },
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
    // header 只读视图经 `NativeProtoExecDataHeaderExt` 门面（unsafe 收口见该 trait 契约）。
    let header = native_proto.header();

    while proto_it != module_protos.len()
      // Safety: `module_protos` 由 bind_module 调用点从已编译模块的存活 `Proto` 指针构造，元素均
      // 非空；`proto_it` 受 `!= len()` 短路保护恒在界内。此处仅读 `bytecodeid` 标量字段。
      && unsafe { (*module_protos[proto_it]).bytecodeid as u32 } != header.bytecode_id
    {
      proto_it += 1;
    }

    CODEGEN_ASSERT!(proto_it != module_protos.len());

    let proto = module_protos[proto_it];

    // Safety: `proto` 同上为存活非空 `*mut Proto`（CODEGEN_ASSERT 已确保索引有效）；写 execdata/
    // exectarget/codeentry 三个字段——`native_proto.as_ptr()` 与静态 `&K_CODE_ENTRY_INSN` 均比 Proto
    // 长寿（execdata 由 SharedCodeAllocator/NativeModule 持有、静态量 'static）。单线程串行、无并发 &mut。
    unsafe {
      (*proto).execdata = native_proto.as_ptr().cast();
      (*proto).exectarget = header.entry_offset_or_address as usize;
      (*proto).codeentry = &K_CODE_ENTRY_INSN;
    }

    protos_bound += 1;
  }

  protos_bound
}
