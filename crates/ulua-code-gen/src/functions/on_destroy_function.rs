use core::ptr::null_mut;

use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::functions::{
  get_code_gen_context::get_code_gen_context,
  get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_destroy_function(l: *mut LuaState, proto: *mut Proto) {
  // Safety: 先判空 l/proto 后返回, 二者其后均活; ctx 仅做存活上下文存在性判定(is_some)。
  // (*proto).execdata 判非空后归还模块引用计数, 满足其对有效 execdata 的前置条件。
  // 随后清空 execdata/exectarget 并令 codeentry=code 均为对本活 Proto 字段的合法写
  // (code 为该 proto 自身字节码数组)。以下窄块统一援引。
  if l.is_null() || proto.is_null() {
    return;
  }

  // Safety: `get_code_gen_context` 依其 `# Safety` 返回存活上下文的独占借用或 None；
  // 本函数不消费上下文内容，仅镜像 cpp 的 `if (getCodeGenContext(L))` 存在性判定，
  // 借用即时结束，不与后续 Proto 字段写冲突。
  let ctx = unsafe { get_code_gen_context(l) };

  if ctx.is_some() && unsafe { !(*proto).execdata.is_null() } {
    // Safety: execdata 由 VM 在销毁 codegen 生成函数的 execdata 时传入(函数头 `/// # Safety`
    // 约定), 指向存活的 NativeProtoExecData 缓冲; 依"头部先于 instruction_offsets 数组"布局,
    // get_native_proto_exec_data_header 由有效指针算得同分配内非空且对齐的头部地址, 故派生 & 合法。
    let header = unsafe { &*get_native_proto_exec_data_header((*proto).execdata as *const u32) };
    // Safety: codegen 产出的 execdata 在 NativeModule::bind_native_protos/rebind_header_module_pointers
    // 中必已填入所属模块地址, 故 native_module 非空; release() 只在 &self 上递减
    // 原子引用计数, 所指 NativeModule 由计数托管存活, 无并存别名冲突。
    unsafe {
      header
        .native_module
        .as_ref()
        .expect("native_module 恒为 Some：绑定模块后才分配 execdata，销毁前必已填入所属模块")
        .release();
    }
  }

  // Safety: 依上契约——均为对本活 Proto 字段的合法写。
  unsafe {
    (*proto).execdata = null_mut();
    (*proto).exectarget = 0;
    (*proto).codeentry = (*proto).code;
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn on_destroy_function_export(l: *mut LuaState, proto: *mut Proto) {
  // Safety: 导出 C ABI 入口原样转发 l/proto 给同契约 unsafe fn on_destroy_function; 该内部
  // 函数对 l/proto 判空早返回并仅在 execdata 非空时使用, 满足被调前置条件。
  unsafe { on_destroy_function(l, proto) };
}
