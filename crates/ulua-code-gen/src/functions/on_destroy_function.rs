use core::ptr::null_mut;

use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::functions::{
  get_code_gen_context::get_code_gen_context,
  get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
};

/// 函数销毁钩子（ecb.destroy 槽位实现）：归还原生码模块引用计数并把 proto 复位到字节码入口。
///
/// # Safety
/// `extern "C-unwind"` FFI 边界，由 VM 宿主按 Lua/C 契约传入存活的 `LuaState` 与 `Proto`。
/// 判空早返回后二者均活；`get_code_gen_context` 依其 `# Safety` 返回存活上下文的独占借用或 None，
/// 本函数只做存在性判定（镜像 cpp `if (getCodeGenContext(L))`），借用即时结束，不与后续 Proto
/// 字段写冲突。`(*proto).execdata` 判非空后指向 `NativeProtoExecData` 存活缓冲，依「头部先于
/// instruction_offsets 数组」布局，`get_native_proto_exec_data_header` 由有效指针算得同分配内非空
/// 且对齐的头部地址，故派生 `&` 合法；`release()` 只在 `&self` 上递减原子引用计数，无并存别名。
/// 随后清空 `execdata`/`exectarget` 并令 `codeentry = code` 均为对本活 Proto 字段的合法写。
pub unsafe extern "C-unwind" fn on_destroy_function(l: *mut LuaState, proto: *mut Proto) {
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
    // 中必已填入所属模块（故 `native_module` 恒 `Some`）；release() 只在 &self 上递减
    // 原子引用计数, 所指 NativeModule 由计数托管存活, 无并存别名冲突。
    unsafe {
      header
        .native_module
        .expect("native_module 恒为 Some：绑定模块后才分配 execdata，销毁前必已填入所属模块")
        .as_ref()
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
