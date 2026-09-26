//! `luau_compile` → `luau_load` 的一次性样板：对应 cpp 各 codegen 用例里逐字重复的
//! `char* bytecode = luau_compile(...); int result = luau_load(...); free(bytecode);
//! REQUIRE(result == 0);`（如 `tests/Conformance.test.cpp:5205-5208`、`5033-5036`）。

use core::{
  ffi::c_char,
  ptr::{from_mut, null_mut},
  slice::from_raw_parts,
};

use ulua_compiler::{
  functions::luau_compile::luau_compile, records::compile_options::CompileOptions,
};
use ulua_vm::{functions::luau_load::luau_load, records::lua_state::LuaState};

use crate::common::functions::c_alloc::c_free;

/// 编译 `source` 并把它作为 `chunkname` 加载到 `l`，失败即中止用例。
///
/// `options` 为 `None` 时传空指针，走 `luau_compile` 内部缺省编译选项
/// （cpp 的 `luau_compile(source, size, nullptr, &size)`）。
///
/// # Safety
/// 调用方须保证 `l` 是存活且可加载字节码的 `LuaState`（上游同一前提）。
pub unsafe fn compile_and_load(
  l: *mut LuaState,
  source: &str,
  chunkname: &str,
  options: Option<&mut CompileOptions>,
) {
  let options = match options {
    Some(options) => from_mut(options),
    // FFI: c-API 要求 NULL
    None => null_mut(),
  };

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`source` 由调用方持有、字节区间合法，`options` 为
  // null（走编译端缺省）或存活的可写 `CompileOptions`；`luau_compile` 写出
  // `bytecode_size` 并返回产物缓冲（失败时也可能为空，下一行即断言）。
  let mut bytecode_size = 0usize;
  let bytecode = unsafe {
    luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      options,
      &mut bytecode_size,
    )
  };
  assert!(!bytecode.is_null(), "luau_compile failed for {chunkname:?}");

  // luau_compile 的裸缓冲 + 长度出参是本文件唯一的 C 边界：转成切片交给
  // `luau_load`（cpp 的 `bytecode.data()/bytecode.size()` 同形态）。
  // Safety: 上一行已断言非空，luau_compile 保证 `bytecode_size` 个字节可读、
  // 至本函数末尾 c_free 前存活。
  let bytes = unsafe { from_raw_parts(bytecode.cast::<u8>(), bytecode_size) };
  // Safety: `l` 是存活且可加载字节码的状态（函数级契约），`bytes` 为上一步物化的合法切片。
  let result = unsafe { luau_load(l, chunkname, bytes, 0) };
  // Safety: `bytecode` 是 luau_compile 交回、尚未释放的裸缓冲。
  // cpp 用 `std::string` 承载 RAII；本端口 `luau_compile` 返回裸缓冲，
  // 故先释放再断言，保证加载失败时不泄漏。
  unsafe { c_free(bytecode.cast()) };
  assert_eq!(0, result, "luau_load failed for {chunkname:?}");
}
