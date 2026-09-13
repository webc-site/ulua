use core::ffi::c_char;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::BytecodeEncoder;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{functions::luau_load::luau_load, type_aliases::lua_state::lua_State};

use crate::functions::lua_string::lua_string;

/// 对应 C++ `loadFromSource`：编译源码并加载进 VM。
pub(crate) fn load(l: *mut lua_State, source: &str) -> Option<String> {
  struct NoopEncoder;

  impl BytecodeEncoder for NoopEncoder {
    fn encode(&mut self, _data: &mut [u32]) {}
  }

  let options = CompileOptions::default();
  let parse_options = ParseOptions::default();
  let mut encoder = NoopEncoder;
  let bytecode = compile(
    source,
    &options,
    &parse_options,
    &mut encoder as *mut dyn BytecodeEncoder,
  );

  load_bytecode(l, bytecode.as_bytes())
}

/// 对应 C++ `loadFromBytecode`：加载预编译字节码，失败时返回栈顶错误信息。
pub(crate) fn load_bytecode(l: *mut lua_State, bytecode: &[u8]) -> Option<String> {
  unsafe {
    if luau_load(
      l,
      c"=config".as_ptr(),
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    ) != 0
    {
      return Some(lua_string(l, -1));
    }
  }

  None
}
