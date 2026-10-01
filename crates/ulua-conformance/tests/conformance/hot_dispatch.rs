// 热层派发（become 尾调用链）定向用例：本仓新增，cpp/tests/conformance 无对应源。
//
// 被测对象是 `crates/ulua-vm/src/functions/luau_execute.rs` 的热臂集合（HOT_ARMS）与
// 条件跳转的 `jump_split!` 双尾写法，断言分两层：
//
// * `hotdispatch.luau`：纯 Lua 断言——热臂内抛错的展开、抛错后状态自检、协程跨热臂
//   yield/resume、回边边界形状（零次/负步长/NaN/±inf）、元表 `__index`/`__newindex` 抛错。
//   以 `run_fixture` 运行，解释器与原生（LUAU_CODEGEN=1）两种模式都必须成立。
// * `hotdispatch_hook.luau`：Rust 侧写入 `callbacks.interrupt` 后驱动热循环，钉热臂在钩子
//   激活时经 `backedge_idle` 判定整臂 `become` 到冷续延之后的语义——回边命中数、钩子里
//   `lua_yield` 的续跑、钩子里 `luaL_error` 的展开。以 skipCodegen 运行（被测对象只有
//   解释器派发环），并把每次命中的 `currentline` 钉在对应函数源码行区间内。

use core::{
  ffi::c_int,
  sync::atomic::{AtomicI32, Ordering},
};

use ulua_compiler::records::compile_options::CompileOptions;
use ulua_vm::{
  enums::lua_status::LuaStatus, luaL_error, macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::LuaState,
};

use crate::common::functions::{
  default_compile_options::default_compile_options,
  run_conformance::{run_conformance, run_fixture},
  safe_api::{
    callbacks_mut, getinfo, l_checkinteger, l_checkstring_bytes, newthread, pop, pushinteger,
    resume, state_mut, yield_, zero_debug,
  },
};

/// 钩子行为：只计数 / 在触发点让出 / 在触发点抛错。
const MODE_COUNT: i32 = 0;
const MODE_YIELD: i32 = 1;
const MODE_RAISE: i32 = 2;

/// 钩子记账（进程级静态，与 `CONFORMANCE_INTERRUPT_STATE` 同款：一次只跑一个用例块，
/// 每块前 `reset`）。
struct HotHookState {
  mode: AtomicI32,
  /// 触发点：第几次命中执行 mode 动作，0 = 从不触发
  trigger: AtomicI32,
  index: AtomicI32,
  line_min: AtomicI32,
  line_max: AtomicI32,
}

impl HotHookState {
  const fn new() -> Self {
    Self {
      mode: AtomicI32::new(MODE_COUNT),
      trigger: AtomicI32::new(0),
      index: AtomicI32::new(0),
      line_min: AtomicI32::new(i32::MAX),
      line_max: AtomicI32::new(i32::MIN),
    }
  }

  fn reset(&self, mode: i32, trigger: i32) {
    self.mode.store(mode, Ordering::SeqCst);
    self.trigger.store(trigger, Ordering::SeqCst);
    self.index.store(0, Ordering::SeqCst);
    self.line_min.store(i32::MAX, Ordering::SeqCst);
    self.line_max.store(i32::MIN, Ordering::SeqCst);
  }

  fn hits(&self) -> i32 {
    self.index.load(Ordering::SeqCst)
  }
}

static HOT_HOOK: HotHookState = HotHookState::new();

/// `callbacks.interrupt` 钩子：每次命中记一行号区间，命中 `trigger` 时让出或抛错。
///
/// # Safety
///
/// 由 VM 在受保护帧的中断点调用：`l` 为当前活跃线程，`gc >= 0` 的调用属 GC 阶段
/// （与派发层无关，直接返回），与 cpp `Interrupt` 用例的回调契约一致。
unsafe extern "C-unwind" fn hot_dispatch_interrupt(l: *mut LuaState, gc: c_int) {
  if gc >= 0 {
    return;
  }

  let index = HOT_HOOK.index.fetch_add(1, Ordering::SeqCst) + 1;

  // 命中点的源码行号必须落在被驱动函数的区间内：这条断言把「回边上的 pc 推进」钉死，
  // jump_split! 的两条尾块若把 pc 走错，行号会跑到函数外（或直接命中失败）。
  let mut ar = zero_debug();
  getinfo(l, 0, b"l\0", &mut ar);
  let line = ar.currentline;
  if line >= 0 {
    HOT_HOOK.line_min.fetch_min(line, Ordering::SeqCst);
    HOT_HOOK.line_max.fetch_max(line, Ordering::SeqCst);
  }

  let trigger = HOT_HOOK.trigger.load(Ordering::SeqCst);
  if trigger == 0 || index != trigger {
    return;
  }

  match HOT_HOOK.mode.load(Ordering::SeqCst) {
    // 与 `conformance_interrupt_interrupt` 同一口径：中断点让出 / 抛错。
    MODE_YIELD => {
      yield_(l, 0);
    }
    MODE_RAISE => {
      // Safety: `l` 为存活受保护帧；`luaL_error` 走本仓 longjmp 模拟（panic），不返回。
      unsafe { luaL_error!(l, "timeout") };
    }
    _ => {}
  }
}

/// 在新线程上以单个整数实参调用全局函数 `name`，断言跑完后返回栈顶整数结果。
///
/// 读取前先断言状态：出错/让出时线程栈上没有结果值，此时 `checkinteger(-1)` 会踩到
/// `index_2_addr` 的越界哨兵（空栈读），断言消息也就不成其为本用例真正要说的话。
fn call_int(l: *mut LuaState, name: &'static str, arg: c_int) -> c_int {
  let t = newthread(l);
  state_mut(t).get_field_str(LUA_GLOBALSINDEX, name);
  pushinteger(t, arg);
  let st = resume(t, None, 1);
  assert_eq!(st, LuaStatus::Ok as c_int, "{name} must run to completion");
  let value = l_checkinteger(t, -1);
  pop(t, 1);
  value
}

/// 同 [`call_int`]，结果为字符串。
fn call_str(l: *mut LuaState, name: &'static str, arg: c_int) -> String {
  let t = newthread(l);
  state_mut(t).get_field_str(LUA_GLOBALSINDEX, name);
  pushinteger(t, arg);
  let st = resume(t, None, 1);
  assert_eq!(st, LuaStatus::Ok as c_int, "{name} must run to completion");
  let value = String::from_utf8_lossy(l_checkstring_bytes(t, -1)).into_owned();
  pop(t, 1);
  value
}

/// 核对钩子观察到的行号区间落在 `[lo, hi]`（含），并回报命中数。
fn check_lines(lo: i32, hi: i32, what: &str) -> i32 {
  let min = HOT_HOOK.line_min.load(Ordering::SeqCst);
  let max = HOT_HOOK.line_max.load(Ordering::SeqCst);
  assert!(
    min >= lo && max <= hi,
    "{what}: interrupt hits observed outside lines {lo}..={hi} (got {min}..{max})"
  );
  HOT_HOOK.hits()
}

#[test]
fn conformance_hot_dispatch() {
  run_fixture("hotdispatch.luau");
}

#[test]
fn conformance_hot_dispatch_hook() {
  // 与 `conformance_interrupt` 同因：固定 optimization_level = 1，回边指令序列（以及
  // 每次命中的 currentline）才与这里写死的区间一一对应。
  let copts = CompileOptions {
    optimization_level: 1,
    ..default_compile_options()
  };

  // skip_codegen = true：本用例测的是解释器 become 尾调用派发层。
  let global_state = run_conformance(
    "hotdispatch_hook.luau",
    None,
    None,
    None,
    Some(&copts),
    true,
    None,
  );
  let l = global_state.as_ptr();

  callbacks_mut(l).interrupt = Some(hot_dispatch_interrupt);

  // ① FORNLOOP 回边：10 圈求和 55，每次回边都命中钩子，命中行号全在 hotforn 体内
  HOT_HOOK.reset(MODE_COUNT, 0);
  assert_eq!(
    call_int(l, "hotforn", 10),
    55,
    "sum across hot backedges with an interrupt hook installed"
  );
  let hits = check_lines(12, 18, "hotforn");
  assert!(
    hits >= 10,
    "every FORNLOOP backedge must reach the hook, hits={hits}"
  );

  // ② while（JUMPIFNOT + JUMPBACK）：8 圈求和 36
  HOT_HOOK.reset(MODE_COUNT, 0);
  assert_eq!(call_int(l, "hotwhile", 8), 36);
  let hits = check_lines(20, 28, "hotwhile");
  assert!(hits >= 8, "JUMPBACK must reach the hook, hits={hits}");

  // ③ 嵌套循环 + 寄存器/常量混合表访问（GETTABLEN/SETTABLEN/MODK）
  HOT_HOOK.reset(MODE_COUNT, 0);
  assert_eq!(call_int(l, "hotnested", 12), 24);
  check_lines(30, 38, "hotnested");

  // ④ 钩子在第 3 次命中让出：线程停在热臂中间，续跑后按原状态跑完（savedpc 回写）
  HOT_HOOK.reset(MODE_YIELD, 3);
  let t = newthread(l);
  state_mut(t).get_field_str(LUA_GLOBALSINDEX, "hotforn");
  pushinteger(t, 10);
  let st = resume(t, None, 1);
  assert_eq!(
    st,
    LuaStatus::Yield as c_int,
    "hook yield must suspend mid-loop"
  );
  assert_eq!(HOT_HOOK.hits(), 3);
  let st = resume(t, None, 0);
  assert_eq!(st, LuaStatus::Ok as c_int);
  assert_eq!(
    l_checkinteger(t, -1),
    55,
    "resumed loop must finish the same sum"
  );
  pop(t, 1);

  // ⑤ 钩子在第 5 次命中抛错：错误由循环内层 pcall 接住，展开后同线程继续跑热循环
  //（hotpcall 内部再调 hotforn(3)，此时钩子只计数不再抛错）
  HOT_HOOK.reset(MODE_RAISE, 5);
  assert_eq!(call_str(l, "hotpcall", 10), "caught");
  check_lines(12, 60, "hotpcall");

  // 摘钩后回到纯热臂路径，结果与①一致
  callbacks_mut(l).interrupt = None;
  HOT_HOOK.reset(MODE_COUNT, 0);
  assert_eq!(call_int(l, "hotforn", 10), 55);
  assert_eq!(HOT_HOOK.hits(), 0, "hook must not fire after being cleared");
}
