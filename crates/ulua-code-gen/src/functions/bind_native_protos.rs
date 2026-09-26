use ulua_common::enums::luau_opcode::LuauOpcode;
use ulua_vm::{records::proto::Proto, type_aliases::instruction::Instruction};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
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
    // Safety: `native_proto` 是 `NonNull<u32>`，指向 create_native_proto_exec_data 一次性分配的
    // instruction_offsets 数组；`get_native_proto_exec_data_header` 从数组首地址反偏 header 大小
    // 回到同一分配内的 header（见该函数证成），故结果非空且对齐，`&*` 只读借用无 &mut 别名。
    let header = unsafe { &*get_native_proto_exec_data_header(native_proto.as_ptr()) };

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
