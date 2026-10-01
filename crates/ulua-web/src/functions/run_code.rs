//! `static std::string runCode(LuaState* l, const std::string& source)`
//! (`CLI/src/Web.cpp:71-140`) 的薄壳。
//!
//! Compiles `source` and delegates the load/run/print/error assembly to
//! [`ulua_vm::functions::run_loaded_chunk`]（web 形态不经 `_PRETTYPRINT`，传
//! `pretty_print_fallback = false`），仅在失败出口拼接 cpp 同款
//! `short_src:line` 错误前缀。

use core::ptr::{from_mut, null, null_mut};
use std::string::String;

use itoa::Buffer;
use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_common::functions::c_str::{cstr, cstr_cow};
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{
  functions::{lua_getinfo::lua_getinfo, run_loaded_chunk::run_loaded_chunk},
  macros::lua_idsize::LUA_IDSIZE,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

/// `lua_getinfo` 选项串：source/short_src/line（NUL 结尾字节串，收口点转 C 指针）。
const GETINFO_SLN_OPT: &[u8] = b"sln\0";

/// cpp `LuaDebug ar;` 的零初值（`Web.cpp:116`）：整块在编译期成形，
/// 免 `mem::zeroed()` 的 unsafe 与「POD 全零合法」的口头论证。
/// `ssbuf` 长度取 VM 的 `LUA_IDSIZE`（`luaconf.h:71` = 256），不重复字面量。
///
/// FFI: c-API 要求 NULL —— `LuaDebug` 是 VM 的 C ABI 镜像结构
/// （`ulua_vm::records::lua_debug::LuaDebug`，`#[repr(C)]`，出处
/// `VM/include/lua.h:488-502`），指针字段的本体即 C 侧 `char*`/`void*`，
/// 全零初值是「出参未填即空」的 cpp 观察语义（`name/what/source/short_src`
/// 判 null、`userdata` 宿主回调判 nullptr）。该结构声明与 `luau_callhook` 写端
/// 均在 ulua-vm（本 crate 外），单端改 `Option<&CStr>` 形态不成立；此处
/// `null()`/`null_mut()` 仅作 const POD 初值，从不被本侧解引用。
const ZERO_DEBUG: LuaDebug = LuaDebug {
  name: null(),
  what: null(),
  source: null(),
  short_src: null(),
  linedefined: 0,
  currentline: 0,
  protoid: 0,
  bytecodeid: 0,
  nupvals: 0,
  nparams: 0,
  isvararg: 0,
  userdata: null_mut(),
  ssbuf: [0; LUA_IDSIZE as usize],
};

/// 运行 `source`，返回装配后的结果/错误文本（成功为空串，cpp 同形）。
///
/// `l` 以 `&mut LuaState` 表达「存活且本帧独占」的 VM 状态：原
/// `# Safety`（non-null、已初始化）契约由引用类型承接，调用方只需提供
/// 独占借用（`run_in_sandbox` 判空后的守卫态、测试 fixture）。
pub fn run_code(l: &mut LuaState, source: &str) -> String {
  let bytecode = compile(
    source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  // Safety: `l` 来自调用方的独占可变借用，指针由 `from_mut` 就地派生（不越过
  // 借用窗口使用）；run_loaded_chunk 的前置（活跃状态机、调用前栈平衡）由
  // `run_in_sandbox` 的建立序列与本函数「每次调用栈自平衡」的使用约定成立。
  let result = unsafe { run_loaded_chunk(from_mut(l), &bytecode, false) };

  // 加载失败（`Err` 为原始消息，不带前缀）与运行失败（`Err` 已含消息 + 回溯）
  // 在 web 形态的区分恰好由前缀的 getinfo 结果保证：`lua_getinfo(l, 0, ..)`
  // 的 level 0 分支要求宿主状态存在活跃调用帧（`0 < ci 相对 base_ci 的深度`），
  // 而本函数的两个入口（execute_script / wasm run）传入的都是无活跃帧的新建
  // 状态，故 getinfo 恒返回 0、前缀恒为空串——与 cpp 在 load 失败路径
  // 不拼前缀、在 resume 失败路径拼到空前缀的观察行为逐字节一致。
  match result {
    Ok(()) => String::new(),
    Err(error) => {
      // LuaDebug ar;
      // if (lua_getinfo(l, 0, "sln", &ar))
      let mut ar = ZERO_DEBUG;
      // Safety: `l` 由参数借用保证为活跃状态机（指针就地派生，不越过借用窗口）；
      // `ar` 是可写局部 POD，`"sln"` 选项只写字段、不压栈。
      if unsafe { lua_getinfo(from_mut(l), 0, cstr(GETINFO_SLN_OPT), &mut ar) } != 0 {
        // 前缀拼在 run_loaded_chunk 返回的消息之前，与 cpp 同序。
        // Safety: getinfo 成功时 short_src 是 ar 内拥有的 NUL 结尾缓冲。
        let short_src = unsafe { cstr_cow(ar.short_src) };
        let mut itoa_buf = Buffer::new();
        let line = itoa_buf.format(ar.currentline);
        // 按前缀与错误消息实际长度精确预分配，免二次扩容；
        // 若 getinfo 返回 0 则直接返回原始 error，零堆分配。
        let prefix_len = short_src.len() + 1 + line.len() + 2;
        let mut prefixed = String::with_capacity(prefix_len + error.len());
        prefixed.push_str(&short_src);
        prefixed.push(':');
        // itoa 栈缓冲直拼，免 `to_string()` 的堆分配；与 `core::fmt` 逐字节一致
        prefixed.push_str(line);
        prefixed.push_str(": ");
        prefixed.push_str(&error);
        prefixed
      } else {
        error
      }
    }
  }
}
