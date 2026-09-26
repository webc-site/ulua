//! lvmexecute 快速调用/算术臂契约测试（对照 cpp/VM/src/lvmexecute.cpp）：
//! 1. `LOP_FASTCALL1` 的 `n >= 0` 命中路径：`luauF_mathmodf` 直接把
//!    `res = ip`、`res + 1 = fp` 写进 CALL 的 ra 区，并跳过回退指令与 CALL；
//! 2. `LOP_FASTCALL1` 的 `n < 0` 回退路径：未移植槽位（`luau_f_missing`）
//!    落到编译器伴随发出的慢路径（GETGLOBAL + GETTABLEKS + CALL）；
//! 3. `LOP_LOADK / LOP_ADD / LOP_RETURN` 基本算术臂：双数值快路径结果回拷宿主栈。
//!
//! 字节码手工拼装（v9，`LBC_VERSION_TARGET` + typesversion 2，字段顺序对照
//! cpp/VM/src/lvmload.cpp 的 `readHeader`/`readProto`），经 `luau_load` 装载后
//! 用 `lua_pcall` 真实执行。指令编码对照 cpp Common/include/Luau/Bytecode.h：
//! op = 位 0..7，A = 位 8..15，B = 位 16..23，C = 位 24..31，D = 位 16..31 符号扩展。

use ulua_common::enums::{
  luau_builtin_function::LuauBuiltinFunction, luau_bytecode_tag::LuauBytecodeTag,
  luau_opcode::LuauOpcode,
};
use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_l_openlibs::lua_l_openlibs, lua_pcall::lua_pcall,
    lua_setsafeenv::lua_setsafeenv, lua_tolstring::lua_tolstring_ref, lua_tonumberx::lua_tonumberx,
  },
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
};

#[path = "common/mod.rs"]
mod common;
#[path = "common/state.rs"]
mod state;

use common::{Blob, ProtoSpec, load, proto_blob};
use state::State;

/// 本文件用例的固定 chunkname（load 错误消息前缀）
const CHUNK: &str = "vm_execute.test";

const CONSTANT_NUMBER: u8 = LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8;
const CONSTANT_STRING: u8 = LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8;

const fn insn(op: LuauOpcode, a: u32, b: u32, c: u32) -> u32 {
  (op as u32) | (a << 8) | (b << 16) | (c << 24)
}

/// LOADK：A = 目标寄存器，D（位 16..31，小索引时为正）= 常量表下标
const fn loadk(a: u32, k: u32) -> u32 {
  insn(LuauOpcode::LOP_LOADK, a, 0, 0) | (k << 16)
}

/// 本文件用例的固定 proto 形态：单 main proto、maxstacksize 8、无 debugname
/// （字段展开见 tests/common 的 `ProtoSpec::write`）
fn spec<'a>(code: &'a [u32], constants: &'a [u8], nconstants: u32) -> ProtoSpec<'a> {
  ProtoSpec {
    code,
    constants,
    nconstants,
    maxstacksize: 8,
    ..ProtoSpec::default()
  }
}

/// 本文件用例的专属辅助，挂在共享的 `state::State` 上
impl State {
  /// 打开全局表的 safeenv 门（lvmexecute.cpp 的 `cl->env->safeenv` 分支，
  /// 与 cpp `luaL_sandboxthread` 的放行顺序一致）
  fn enable_safeenv(&self) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self.l` 在本用例作用域内存活，`lua_setsafeenv` 仅写全局表 flag。
    unsafe { lua_setsafeenv(self.l, LUA_GLOBALSINDEX, 1) };
  }

  /// 调用栈顶闭包并断言成功；失败时带上栈顶错误消息
  unsafe fn pcall_ok(&self, nresults: i32) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self.l` 在本用例作用域内存活，`lua_pcall` 处于受保护帧。
    let status = unsafe { lua_pcall(self.l, 0, nresults, 0) };
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self.l` 在本用例作用域内存活，仅失败路径读栈顶错误串。
    assert_eq!(status, 0, "执行失败: {}", unsafe { self.top_message() });
  }

  /// 栈指定位的 number 结果（非数值即失败）
  unsafe fn number_at(&self, index: i32) -> f64 {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self.l` 在本用例
    // 作用域内存活，`lua_tonumberx` 只读栈槽。
    unsafe { lua_tonumberx(self.l, index) }.unwrap_or_else(|| panic!("栈位 {index} 必须是数值结果"))
  }

  /// 栈顶按错误消息读取；仅在断言失败路径调用
  unsafe fn top_message(&self) -> String {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self.l` 在本用例
    // 作用域内存活，`lua_tolstring_ref` 只读栈槽并按切片带出全部字节（UTF-8 近似）。
    match unsafe { lua_tolstring_ref(self.l, -1) } {
      Some(s) => String::from_utf8_lossy(s).into_owned(),
      None => String::from("<非字符串错误>"),
    }
  }
}

/// FASTCALL1 命中路径（lvmexecute.cpp:3149）：`luauF_modf` 成功后
/// `pc += skip + 1` 同时跳过回退指令与 CALL，结果按 `res = ip`、`res + 1 = fp`
/// 落在 CALL 的 ra 区（lbuiltins.cpp:330 的次序）。
#[test]
fn fastcall1_modf_fast_path_returns_ip_then_fp() {
  let s = State::new();
  s.enable_safeenv();

  let mut constants = vec![CONSTANT_NUMBER];
  constants.extend_from_slice(&3.5f64.to_le_bytes());

  let code = [
    loadk(1, 0), // r1 = k0 = 3.5
    insn(
      LuauOpcode::LOP_FASTCALL1,
      LuauBuiltinFunction::LbfMathModf as u32,
      1,
      1,
    ), // 实参 r1，CALL 在 pc(2)+1
    insn(LuauOpcode::LOP_LOADNIL, 2, 0, 0), // 回退占位：命中路径必须不执行
    insn(LuauOpcode::LOP_CALL, 0, 2, 3), // ra = r0，1 实参、2 结果
    insn(LuauOpcode::LOP_RETURN, 0, 3, 0), // 返回 r0、r1 两个值
  ];

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    assert_eq!(
      load(
        s.l,
        CHUNK,
        &proto_blob(&[], &[spec(&code, &constants, 1)], 0)
      ),
      0
    );
    assert_eq!(lua_gettop(s.l), 1, "load 后栈顶必须是 main 闭包");
    s.pcall_ok(2);
    assert_eq!(s.number_at(-2), 3.0, "上游次序：res = ip");
    assert_eq!(s.number_at(-1), 0.5, "上游次序：res + 1 = fp");
  }
}

/// FASTCALL1 回退路径：未移植槽位（cpp `luauF_table` 尾部 dummy →
/// `luau_f_missing` 恒返 -1）必须继续执行编译器伴随发出的慢路径指令序列
/// `GETGLOBAL math; GETTABLEKS .abs; CALL`，即手拼出 `math.abs(-3.5)` 的
/// 编译产物形态，结果精确为 3.5。
#[test]
fn fastcall1_missing_slot_falls_back_to_env_lookup() {
  let s = State::new();
  unsafe { lua_l_openlibs(s.l) };
  s.enable_safeenv();

  // 常量段含 varint 负载，直接用 common 的 `Blob` 写入器拼装
  let constants = {
    let mut c = Blob::default();
    c.byte(CONSTANT_NUMBER)
      .raw(&(-3.5f64).to_le_bytes())
      .byte(CONSTANT_STRING)
      .varint(1) // k1 = "math"（字符串 id 按 strings[id - 1] 解析）
      .byte(CONSTANT_STRING)
      .varint(2); // k2 = "abs"
    c
  };

  let code = [
    loadk(1, 0), // r1 = -3.5
    insn(
      LuauOpcode::LOP_FASTCALL1,
      LuauBuiltinFunction::LbfMathAbs as u32,
      1,
      4,
    ), // CALL 在 pc(2)+4 = 下标 6
    insn(LuauOpcode::LOP_GETGLOBAL, 0, 0, 0), // 2: r0 = env["math"]（C = 预测槽，0 走慢路径）
    1,           // 3: AUX = 常量下标 1
    insn(LuauOpcode::LOP_GETTABLEKS, 0, 0, 0), // 4: r0 = r0["abs"]
    2,           // 5: AUX = 常量下标 2
    insn(LuauOpcode::LOP_CALL, 0, 2, 2), // 6: r0(r1) → r0，1 结果
    insn(LuauOpcode::LOP_RETURN, 0, 2, 0), // 7: 返回 1 个值 r0
  ];

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`constants` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    assert_eq!(
      load(
        s.l,
        CHUNK,
        &proto_blob(&[b"math", b"abs"], &[spec(&code, &constants.bytes, 3)], 0)
      ),
      0
    );
    s.pcall_ok(1);
    assert_eq!(s.number_at(-1), 3.5, "回退慢路径必须命中真实 math.abs");
  }
}

/// `LOP_ADD` 双数值快路径（lvmexecute.cpp:1764）与 LOADK/RETURN 的返回值
/// 回拷：2.5 + 4.0 = 6.5，不依赖 fastcall 与 safeenv 门。
#[test]
fn loadk_add_return_executes_numeric_fast_path() {
  let s = State::new();

  let mut constants = vec![CONSTANT_NUMBER];
  constants.extend_from_slice(&2.5f64.to_le_bytes());
  constants.push(CONSTANT_NUMBER);
  constants.extend_from_slice(&4.0f64.to_le_bytes());

  let code = [
    loadk(1, 0),                           // r1 = 2.5
    loadk(2, 1),                           // r2 = 4.0
    insn(LuauOpcode::LOP_ADD, 0, 1, 2),    // r0 = r1 + r2
    insn(LuauOpcode::LOP_RETURN, 0, 2, 0), // 返回 1 个值 r0
  ];

  unsafe {
    assert_eq!(
      load(
        s.l,
        CHUNK,
        &proto_blob(&[], &[spec(&code, &constants, 2)], 0)
      ),
      0
    );
    s.pcall_ok(1);
    assert_eq!(s.number_at(-1), 6.5);
  }
}
