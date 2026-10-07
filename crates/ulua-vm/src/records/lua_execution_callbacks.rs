//! 执行回调表（JIT/code-gen 挂点）。
//!
//! 回调签名保持 `extern "C-unwind"` 函数指针形态：ulua-code-gen 的 JIT 与
//! conformance 测试以同一 ABI 约定安装/调用它们（unwind 跨回调帧是显式契约），
//! 因此本表不是业务逻辑指针，而是 crate 内部的稳定回调 ABI，非 §2 待消灭对象。
//! 各槽位 `None` 表示该回调未安装（cpp 的 NULL 槽），空槽判定一律用 `Option`。

use core::{
  ffi::{c_char, c_void},
  ptr::{NonNull, null_mut},
  sync::atomic::AtomicPtr,
};

use crate::records::{closure::Closure, lua_state::LuaState, proto::Proto};

/// FORN trace 层回边计数的进程级武装槽（T3 单载荷门）：非空 = 有候选位点在
/// 计数（值 = 注册表持有的计数单元地址，Box 稳定）。解释器 `h_fornloop`
/// 逐回边的唯一在位知识——一次链深 0 的静态读 + 非空快检（与 fflag 静态读
/// 同一 absorbed 等级）；原「旗标读 + `l→global→ecb` 两级依赖装载」整支移出
/// 热路。武装/解除只由 code-gen 侧（trace_forn_registry）写入：武装仅发生在
/// 旗标开的入口问询路径（非空蕴含旗标开），解除在阈值慢路或 state 关闭。
/// 身份键（proto/pc）留在 per-state `ecb`：跨 state/线程交错时武装态只归
/// 武装 state 所有，异 state 回边在身份比对处静默跳过（不触碰他 state 的
/// 计数单元，无跨线程解引用）。计数是纯热度信号，宽松序下读写竞争最坏
/// 损失一次计数，不影响执行语义。
pub static FORN_HEAT_ARMED: AtomicPtr<u64> = AtomicPtr::new(null_mut());

#[repr(C)]
#[derive(Debug)]
pub struct lua_ExecutionCallbacks {
  /// 回调宿主数据；由安装方（code-gen/conformance）持有生命周期，`None` 语义不存在，
  /// 未安装回调时恒为 null 且不会被解引用。
  pub context: *mut c_void,
  pub close: Option<unsafe extern "C-unwind" fn(l: *mut LuaState)>,
  pub destroy: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto)>,
  pub enter: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto) -> i32>,
  pub disable: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto)>,
  pub getmemorysize:
    Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto) -> usize>,
  pub gettypemapping:
    Option<unsafe extern "C-unwind" fn(l: *mut LuaState, str: *const c_char, len: usize) -> u8>,
  /// 原生码之后的宿主反馈计数向量首址。DELIBERATE DEVIATION / review.md §2/§3：cpp
  /// `CodeGen/IExecutionCallbacks.h` 把它声明成 `char* (*getcounterdata)(...)`，但那
  /// 只是一个**字节地址**（元素为 `kind(u32)+pcpos(u32)+hits(u64)`，见
  /// `functions::getcounters`），从来不是字符串；故 `c_char` 不收在此槽位（它只留给真正
  /// 承载 C 字符串的 `gettypemapping` 槽），并把 cpp 的「返回 NULL 由调用方判空」折叠成
  /// `Option<NonNull<u8>>` 的缺席语义。`repr(C)` 下 `Option<NonNull<u8>>` 与裸指针逐位等价
  /// （niche 优化），ABI（指针宽度/调用约定/空值表示）与 cpp 一致。
  pub getcounterdata: Option<
    unsafe extern "C-unwind" fn(
      l: *mut LuaState,
      proto: *mut Proto,
      count: *mut usize,
    ) -> Option<NonNull<u8>>,
  >,
  pub inlinefunction: Option<
    unsafe extern "C-unwind" fn(
      l: *mut LuaState,
      caller: *mut Closure,
      target: *mut Closure,
      pc: u32,
    ) -> *mut Proto,
  >,
  /// FORN trace 层入口（阶段二 PoC，trace_forn_registry 安装，a64 专属）：
  /// `Some` = 安装方提供「数值 for 环入口裁决 + 整环原生执行」。入参
  /// `(l, proto, fornprep_pcpos)`，由解释器 `h_fornprep` 数值三元组判定通过后
  /// 调用；返回 1 = 已承接（`ci->savedpc` 已落续延 pc——环出口或环内守卫
  /// bail 位点，解释器从 savedpc 重取派发），0 = 不承接（解释器照常走
  /// FORNPREP）。未安装恒 `None`，旗标关时解释器不读本槽。
  pub trace_forn_enter:
    Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto, pcpos: u32) -> i32>,
  /// FORN trace 层回边热度的内联缓存（T2 回边计数；由 code-gen 侧武装/解除，
  /// 解释器 `h_fornloop` 内联消费）。在位信号（计数单元指针）已上收进程级
  /// [`FORN_HEAT_ARMED`] 静态槽（T3 单载荷门，热路零依赖链）；本表保留身份
  /// 键与慢路槽三元组：
  /// - `forn_heat_proto` / `forn_heat_pc`：武装位点的身份键（proto 指针 +
  ///   FORNLOOP pcpos），不匹配 = 静默跳过（计数为纯热度信号，只影响录制
  ///   时机不影响语义；跨 state/线程交错时武装态只归武装 state 所有）；
  /// - `forn_heat_target`：触发录制慢路的精确计数值。
  pub forn_heat_proto: usize,
  pub forn_heat_pc: u32,
  pub forn_heat_target: u64,
  /// FORN trace 层回边阈值慢路：`(l, proto, fornloop_pcpos)`，精确达阈时由
  /// `h_fornloop` 调用（每阈值至多一次）；安装方在内做录制/装配并解除武装。
  pub trace_forn_backedge:
    Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto, pcpos: u32)>,
}

/// 「未安装任何回调」的唯一形态：cpp `L->global->ecb = lua_ExecutionCallbacks{}` 整体清零语义。
///
/// 回调槽用 `Option::None` 表达未安装，数值槽用 0；`context` 是类型擦除的宿主 user-data
/// （安装端写 `code_gen_context as *mut c_void`，读取端 `cast::<T>()` 后 `as_mut()` 还原
/// `Option`），字段类型只能是 `*mut c_void`，null 即「无宿主」。
///
/// 本表由 VM 与 code-gen 两侧重置，此前各自复制了一份全零构造（一份 14 字段字面量、一份
/// `unsafe { zeroed() }`），新增字段时两处都会静默漏配；收口于此，字段清单与定义同址维护。
impl Default for lua_ExecutionCallbacks {
  fn default() -> Self {
    Self {
      context: null_mut(),
      close: None,
      destroy: None,
      enter: None,
      disable: None,
      getmemorysize: None,
      gettypemapping: None,
      getcounterdata: None,
      inlinefunction: None,
      trace_forn_enter: None,
      forn_heat_proto: 0,
      forn_heat_pc: 0,
      forn_heat_target: 0,
      trace_forn_backedge: None,
    }
  }
}

/// Rust 惯用名。原名 `lua_ExecutionCallbacks` 必须保留：`ulua-code-gen` 直接按该名字构造
/// 零值回调表（`repr(C)` 使混合大小写命名豁免 casing lint，改动会波及跨 crate 消费方）。
pub(crate) type LuaExecutionCallbacks = lua_ExecutionCallbacks;
