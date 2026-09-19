use std::ffi::CString;

use ulua_cli_lib::records::lua_state_guard::LuaStateGuard;
use ulua_code_gen::{
  functions::get_assembly::get_assembly,
  records::{assembly_options::AssemblyOptions, lowering_stats::LoweringStats},
};
use ulua_vm::functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load};

/// cpp `getCodegenAssembly` (CLI/src/Compile.cpp:114-129)
///
/// 返回机器码或 asm/IR 文本的字节（cpp 侧同一名 `std::string` 承载两者）。
pub fn get_codegen_assembly(
  name: &str,
  bytecode: &[u8],
  options: AssemblyOptions,
  stats: &mut LoweringStats,
) -> Vec<u8> {
  // luau_load 的 chunkname 需要 NUL 结尾 (命令行参数不含 NUL, 必成功)
  let name_c = CString::new(name).unwrap_or_default();

  let global_state = LuaStateGuard(lua_l_newstate());
  let l = global_state.0;

  if unsafe {
    luau_load(
      l,
      name_c.as_ptr(),
      bytecode.as_ptr() as *const _,
      bytecode.len(),
      0,
    )
  } == 0
  {
    unsafe { get_assembly(l, -1, options, stats) }
  } else {
    eprintln!("Error loading bytecode {name}");
    Vec::new()
  }
}
