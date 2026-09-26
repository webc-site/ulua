use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{functions::luau_load::luau_load, records::lua_state::LuaState};

use crate::functions::lua_string::lua_string;

/// 对应 C++ `loadFromSource`：编译源码并加载进 VM。
///
/// ulua-vm C-API 绑定边界：`l` 契约须为调用方（沙箱 `StateGuard`）持有的
/// 有效 VM 状态，裸指针仅在内部栈操作处使用。
pub(crate) fn load(l: *mut LuaState, source: &str) -> Option<String> {
  let options = CompileOptions::default();
  let parse_options = ParseOptions::default();
  let bytecode = compile(source, &options, &parse_options, NoopEncoder);

  load_bytecode(l, &bytecode)
}

/// 对应 C++ `loadFromBytecode`：加载预编译字节码，失败时返回栈顶错误信息。
///
/// `l` 契约同上：须为调用方持有的有效 VM 状态。
pub(crate) fn load_bytecode(l: *mut LuaState, bytecode: &[u8]) -> Option<String> {
  // Safety: l 为调用方（沙箱 StateGuard）持有的有效 VM 状态；
  // chunkname 与字节码均为本帧存活的借用，luau_load 不在返回后保留其指针
  unsafe {
    // 源名与字节码都按 `&str`/`&[u8]` 直传 `luau_load`（ulua-vm 的 C API 镜像）
    if luau_load(l, "=config", bytecode, 0) != 0 {
      return Some(lua_string(l, -1));
    }
  }

  None
}
