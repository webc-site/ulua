use ulua_common::enums::luau_opcode::LuauOpcode;
use ulua_vm::{records::proto::Proto, type_aliases::instruction::Instruction};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
  records::native_module::NativeModule,
};

static K_CODE_ENTRY_INSN: Instruction = LuauOpcode::LOP_NATIVECALL as u32;

/// 把 native proto 的执行数据绑回 proto（cpp `bindNativeProtos`）。
///
/// `release`（重绑定场景，同步暖重编译专用）：proto 名下的旧 execdata 属旧
/// module，proto 侧关闭回调只会归还当前 execdata（即将指向的新 module）——旧
/// module 的引用计数就此成死账、code_allocator 的 live_allocations 归零断言
/// 必炸。此处把旧 module 逐 proto 转移登记进 `retires`，由上下文关闭链
/// （BaseCodeGenContext::drop，无在途 native 帧）统一归还；立即归还会让在途
/// 执行的旧机器码页被撤执行权限。
pub fn bind_native_protos(
  module_protos: &[*mut Proto],
  native_protos: &mut [NativeProtoExecDataPtr],
  release: bool,
  retires: &mut Vec<*mut NativeModule>,
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
      if release && !(*proto).execdata.is_null() {
        // Safety: execdata 非空即此前绑定成功，header 反查按「header 紧接数组
        // 之前同一分配」布局落界内；旧 module 由计数托管存活，此处只读指针。
        let old = (*get_native_proto_exec_data_header((*proto).execdata.cast())).native_module;
        if !old.is_null() {
          retires.push(old);
        }
      }
      (*proto).execdata = native_proto.as_ptr().cast();
      (*proto).exectarget = header.entry_offset_or_address as usize;
      (*proto).codeentry = &K_CODE_ENTRY_INSN;
    }

    protos_bound += 1;
  }

  protos_bound
}

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  type_aliases::native_proto_exec_data_ptr::{NativeProtoExecDataHeaderExt, NativeProtoExecDataPtr},
};
