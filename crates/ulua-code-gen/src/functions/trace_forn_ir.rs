//! FORN trace 层录制器（阶段二 PoC 片 1+2）：数值 for 闭体（FORNPREP..FORNLOOP）
//! 的线性 IR 录制 + 回边 phi 分类 + 类型特化标注。
//!
//! 蓝图依据（readme/luajit-roadmap.md「全面超越 LuaJIT 总规划」阶段二）：
//! method JIT 线性 IR 无 phi 是跨回边驻留不可达的根因，trace 层以 ZJIT/Maglev
//! 的 block-args 形态带回边 phi——本层的 phi 集即「环头块参数」：
//! - 环控制变量 idx（双精度位型逐位对齐解释器 `setnvalue` 的写回语义）；
//! - 累加器槽（体内在定义点之前被读的 number 槽，如 `sum = sum + ci[j]`）。
//!
//! 环不变槽（只读）以类型特化标注（Table/Number 入口守卫），环体临时以 SSA
//! 临时编号，全部在生成码里驻硬件寄存器（见 trace_forn_codegen_a_64）。
//!
//! 录制 = 静态闭体走查（非动态轨迹）：闭体要求 FORNPREP 的出口恰在 FORNLOOP
//! 之后、FORNLOOP 的回边恰指环体头，环体为直线指令序列（调用/分配/
//! 元方法族一律拒录——降级回解释器，全有全无）。支持集按 matmul 内层 j 环与
//! `sum = sum + ci[j]` 累加形态划定并逐片扩宽：GET/SETTABLE(N)（表源 =
//! 环不变槽或派生表；下标 = 任意 number 操作数）、ADD/SUB/MUL/DIV/MOD(K)、
//! MOVE/LOADN/LOADK（number 常量）、math 单参内建族（sqrt/abs/floor/ceil/
//! round 的 CALL/FASTCALL1 双形态单指令直译，T6 起 sqrt、T7 族泛化）、
//! GETUPVAL/SETUPVAL（T8：闭包捕获变量——变更载体逐访问穿 cell 指针，
//! 只读载体 prologue 内联，nbody `sqrt(dist2)` 的 GETUPVAL+CALL 形态
//! 随之解锁录制资格）。
//!
//! 嵌套表载（族 1，`ts[r][j]` 型）：GETTABLE 结果槽作表用（预扫表位/值位
//! 读面定案）→ TableLoad 派生表——结果 table 驻 x7/w13（单槽），逐迭代
//! 重读全守卫（界/tag==ttable/元表缺席），构造性免别名论证（ts[r] 槽值
//! 可能被别名存储改写，重读即逐位同解释器）；不进同元素证明面（派生对象
//! 可能逐迭代变化）。作值用 → number 特化 ArrayLoad（双读共存拒录——值读
//! 会坠 number 特化 bail 风暴）。
//!
//! 正确性关键（跨回边可见性）：直线体单遍走查的「槽已知值」只覆盖本迭代；
//! 上迭代写在下一迭代全部读点可见。因此槽分类以位次预扫定案——
//! - 读且首写位次 < 首读位次：读点被本迭代写支配 → 临时链（非环携带）；
//! - 读且首写位次 ≥ 首读位次（含算术同指令读写）：读点跨回边吃上一迭代
//!   终值 → 回边 phi（累加器，单写纪律：多写即拒录，保守回解释器）；
//! - 只读：环不变（入口守卫 + 驻留）；
//! - 只写：临时，出口/bail 按终值快照写回。

use alloc::{boxed::Box, vec::Vec};
use core::ptr::addr_of;

use ulua_common::{
  enums::{luau_builtin_function::LuauBuiltinFunction, luau_opcode::LuauOpcode},
  macros::luau_insn_ops::{
    luau_insn_a, luau_insn_aux_a, luau_insn_aux_kv, luau_insn_aux_not, luau_insn_b, luau_insn_c,
    luau_insn_d, luau_insn_op,
  },
};
use ulua_vm::{
  enums::lua_type::LuaType,
  records::{closure::Closure, lua_t_value::TValue},
  type_aliases::{instruction::Instruction, stk_id::StkId},
};

use crate::functions::trace_forn_math_addr::slot_math_fn;

/// 环体指令数上限（PoC 形态护栏：matmul j 环 5 条，留一档余量）
pub(crate) const K_MAX_TRACE_INSTS: usize = 32;
/// 环不变 table 槽上限（生成码固定寄存器规划 x4..x7；有派生表时收缩为 3，
/// x7/w13 让位派生表指针/size）
pub(crate) const K_MAX_TABLES: usize = 4;
/// 派生表上限（x7/w13 单槽：嵌套表载 `ts[r][j]` 的 ts[r] 结果驻留位）
pub(crate) const K_MAX_DERIVED: usize = 1;
/// 环不变 number 槽上限（d6/d7）
pub(crate) const K_MAX_INV_NUMS: usize = 2;
/// 累加器 phi 槽上限（d4/d5）
pub(crate) const K_MAX_ACCS: usize = 2;
/// 环体临时上限（d20..d27 + d28..d31；d8..d15 为 callee-saved 规避域。
/// 临时号按指令递增不复用——3 存多槽体如 nbody 位置环需 12 个临时，
/// 8 上限容量拒录，T5-B 剖面定位后扩 4 槽）
pub(crate) const K_MAX_TEMPS: u8 = 12;
/// number 常量上限（d16..d19）
pub(crate) const K_MAX_CONSTS: usize = 4;
/// 环体引用 upvalue 上限（cell 指针驻留 x14/x15 两槽——X_PC/X_CI 同号，
/// 二者仅出口/bail 块使用，环体内空闲）
pub(crate) const K_MAX_UPVALS: usize = 2;
/// LOADN 立即数入池占位 k 下标（非 k 池引用，identity 排除面）
pub(crate) const K_LOADN_PLACEHOLDER: u32 = u32::MAX;
/// GETTABLEN 表载常量下标入池占位（同上）
pub(crate) const K_NINDEX_PLACEHOLDER: u32 = u32::MAX - 1;

/// 表源（ArrayLoad/ArrayStore/TableLoad/CondLoad 的表操作数）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TableRef {
  /// 环不变槽（入口守卫 tag/元表/readonly + prologue 一次装载）
  Inv(u8),
  /// 派生表（前置 TableLoad 的结果，x7/w13 单槽驻留；逐迭代重读，全守卫，
  /// 不进同元素证明面——对象可能逐迭代变化）
  Derived(u8),
}

/// 操作数值源（类型特化后的静态形态）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TVal {
  /// number 常量（consts 下标；K 形操作数 / LOADN / number LOADK）
  Const(usize),
  /// 环体临时（SSA，寄存器驻留）
  Temp(u8),
  /// 环不变 number 槽（入口守卫 tag==tnumber，跨环驻留）
  InvNum(u8),
  /// 累加器 phi 槽（回边块参数，入口守卫 tag==tnumber）
  Acc(u8),
  /// 环控制变量（FORN idx 的双精度 phi 表示，递增与比较逐位对齐解释器）
  PhiIdx,
  /// math 单参内建的 C 函数槽（槽值 = is_c 闭包且 inner.c.f == 对应内建；
  /// 跨环不变——体无写，入口守卫按槽复检），仅供 CALL 特化消费
  Cfn { slot: u8, kind: TMathUnary },
}

/// 算术种（IEEE 双精度逐位同解释器：fadd/fsub/fmul/fdiv）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TArith {
  Add,
  Sub,
  Mul,
  Div,
  /// 取模：发射 `a - floor(a/b)*b` 四操作序列（fdiv→frintm→fmul→fsub），与
  /// 解释器 `luai_nummod` 的 Rust 形态同序同操作（.floor() 在 a64 落 frintm，
  /// 正确舍入且 rustc 无 FP 缩合契约）——构造性逐位一致，无需守卫/bail。
  Mod,
}

/// 单参单返回 math 内建种（T7 族泛化，按 sqrt 已验证模式逐族扩宽）。
/// 语义面 = Lua math 库同款 f64 运算（math_map1 骨架：`l.check_number(1)`
/// 强制 number 后套 f64 变换压回）——非 number 参数在解释器同执行路径先抛
/// check_number，特化环参数面皆 number 已知值，无守卫面。
/// a64 单指令直译与 Rust f64 方法的语义规格逐一对应（IEEE 754
/// roundToIntegral 族 / abs），位一致构造性成立：
/// - Sqrt → fsqrt（IEEE 正确舍入）；
/// - Abs → fabs（符号位清零；NaN/inf/-0.0 逐位传递）；
/// - Floor → frintm（向 -∞ 舍入 = f64::floor）；
/// - Ceil → frintp（向 +∞ 舍入 = f64::ceil）；
/// - Round → frinta（最近邻 ties-away = f64::round；**非 frintn**——
///   frintn 是 ties-even，与 f64::round 的 ties-away 规格不符，选型
///   以 Rust std 文档语义为准核对汇编器 r1_table 后定 frinta）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TMathUnary {
  Sqrt,
  Abs,
  Floor,
  Ceil,
  Round,
}

/// upvalue 载体种（GETUPVAL 消费面的静态分类；录制期按槽/cell 实测定 Kind）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TUpvalKind {
  /// number 载体：只读 → prologue 内联装载（入口守卫钉 cell 源 tnumber），
  /// 环内零代码；变更 → 逐访问穿 cell 指针（tag 守卫 bail）
  Num,
  /// math 单参内建 C 函数载体（nbody `sqrt(dist2)` 形态）：环内零代码，
  /// CALL/FASTCALL1 特化消费。分类源二途——FASTCALL1 影子内 GETUPVAL 按
  /// bfid 静态种（编译器内建追踪，热路不执行该装载）；影子外按 fn_slots
  /// 槽快照。入口守卫按 cell 源复检 (种)——槽值是跨入口可变的脏快照，
  /// cell 才是真值源
  Cfn(TMathUnary),
}

/// 环体引用的 upvalue（去重，位次 = upvals 下标；TInst 的 `uv` 字段即位次）。
#[derive(Clone, Copy, Debug)]
pub(crate) struct TUpval {
  /// upref 下标（GETUPVAL/SETUPVAL 的 B 字，< nupvalues）
  pub idx: u8,
  /// upref 持真 UpVal（ttisupval：LCT_REF/LCT_UPVAL 捕获）——生成码按此锁
  /// prologue 形态：true 则 cell = `(*UpVal).v`（open→栈槽 / closed→storage，
  /// 逐入口重解）；false 则为 LCT_VAL 值内联 upref（克隆时常量，cell 源 =
  /// upref 本体）。形态由 proto 捕获描述符决定、跨入口稳定，入口守卫仍复核
  pub cell: bool,
  /// 体含 SETUPVAL → cell 指针驻留 + 逐访问穿 cell；否则只读
  pub mutated: bool,
  pub kind: TUpvalKind,
  /// Num+只读：prologue 内联装载的保留临时（环内零代码，槽写走快照写回面）
  pub inv_temp: Option<u8>,
}

/// 单条环体指令。
#[derive(Clone, Copy, Debug)]
pub(crate) enum TInst {
  /// `dst = table[index]`（number 特化载；index 任意 number 操作数——PhiIdx
  /// 走驻留整数快路并可进同元素证明面，其余逐迭代 fcvtzs 往返精确性校验）
  ArrayLoad {
    dst: u8,
    table: TableRef,
    index: TVal,
  },
  /// `dst = table[c]`（0 基立即数下标）
  ArrayLoadN { dst: u8, table: TableRef, c: u8 },
  /// `table[index] = value`（number 特化存；index 语义同 ArrayLoad）
  ArrayStore {
    table: TableRef,
    index: TVal,
    value: TVal,
  },
  /// `table[c] = value`（0 基立即数下标）
  ArrayStoreN { table: TableRef, c: u8, value: TVal },
  /// `slot = src[index]`（派生表装载：结果恒 table，逐迭代重读全守卫——
  /// 界/tag==ttable/元表缺席/(存储穿透时)readonly；值+tag 回写槽保持
  /// 解释器「每迭代覆写」可见面；derived 寄存器 x7/w13 供内层访问）
  TableLoad {
    slot: u8,
    src: TableRef,
    index: TVal,
  },
  /// `dst = table[index]`（布尔载/truthiness 面：只做界守卫——免 number tag
  /// 特化，boolean 载不坠 bail 风暴；值+tag 装载 + 槽回写；dst 紧邻 Branch
  /// 消费（adjacency 由录制面强制），tag 驻 W9 跨发射点存活）
  CondLoad {
    dst: u8,
    slot: u8,
    table: TableRef,
    index: TVal,
  },
  /// `dst = table[c]`（truthiness 面 0 基立即数下标）
  CondLoadN {
    dst: u8,
    slot: u8,
    table: TableRef,
    c: u8,
  },
  /// `dst = lhs op rhs`（dst 恒为环体临时；落槽写经 writebacks 快照）
  Arith {
    dst: u8,
    op: TArith,
    lhs: TVal,
    rhs: TVal,
  },
  /// 区域内 acc 更新 `acc = lhs op rhs`（直写 phi 寄存器 d_acc——跳转未走
  /// 即自然保持旧值，区域 phi 的零选择指令物理形态；回边 fmov 对其豁免）
  AccArith {
    acc: u8,
    op: TArith,
    lhs: TVal,
    rhs: TVal,
  },
  /// 条件跳转（菱形双径直译：条件成立跳 target，否则落穿区域；target 限
  /// 环内前向位点，== 环尾即 continue 语义——落到回边 phi 更新 + 递增 + 判定）
  Branch { cond: TCond, target: u32 },
  /// CALL 特化（math 单参内建族：sqrt/abs/floor/ceil/round）：单指令直译。
  /// dst_slot = 返回槽，guard = Some(func 源槽)（CALL 经 func 槽形态：入口
  /// 复检槽值仍为对应内建）/ None（FASTCALL1 形态：bfid 静态常量，无守卫
  /// 面），arg 皆 number 特化面
  MathUnary {
    dst: u8,
    dst_slot: u8,
    guard: Option<u8>,
    kind: TMathUnary,
    arg: TVal,
  },
  /// `dst = *cell`（T8 变更载体 upvalue 读）：逐访问穿 cell 指针装载（tag
  /// 守卫 bail，类别 6）——跨迭代读写经 cell 传递（SETUPVAL 写 → 下一迭代
  /// GETUPVAL 读），同迭代读后写见新值。只读载体的 GETUPVAL 走 prologue
  /// 内联 / Cfn 零代码面，不经本指令
  UpvalLoad { dst: u8, uv: u8 },
  /// `*cell = value`（T8 upvalue 写）：值 + tnumber tag 穿 cell 双写（open
  /// 态栈写穿透 / closed 态 storage 写，同一 cell 指针语义）；屏障面
  /// `luaC_barriert` 谓词 iscollectable(v) 对 number 恒假 → 构造性零动作
  /// （T7 派生写同款论证）。写效果前向单调：bail 点之前的 SETUPVAL 已生效，
  /// 与解释器从 bail 位点续延的可见面逐位一致（upvalue 写不可快照亦无需
  /// 快照——外部效果无部分副作用问题）
  UpvalStore { uv: u8, value: TVal },
}

/// 数值比较种（fcmp 条件面；NaN 无序语义由发射面 VS 门控逐位对齐解释器）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TCmpOp {
  Lt,
  Le,
  Eq,
}

/// 分支条件形态。
#[derive(Clone, Copy, Debug)]
pub(crate) enum TCond {
  /// 真值判定：generic temp 的 tag 驻 W9（CondLoad 紧邻分支的 adjacency
  /// 由录制面强制），值驻 d_temp；positive = JUMPIF（真跳）/ JUMPIFNOT（假跳）
  Truth { temp: u8, positive: bool },
  /// 数值比较：双 number 操作数（generic temp 拒录）
  Cmp {
    op: TCmpOp,
    lhs: TVal,
    rhs: TVal,
    positive: bool,
  },
  /// 无条件跳转（number 条件真值常量折叠：JUMPIF number 恒真）
  Always,
  /// nil 常量比较（JUMPXEQKNIL）：tag==TNil 判定，操作数为栈槽（tag 面
  /// 读自 base，不依赖紧邻加载）；positive = hit 跳 / negative = 未中跳
  Nil { slot: u8, positive: bool },
}

/// 带源 pc 的环体指令（pc 即该指令守卫失败的 bail 重入位点）。
#[derive(Clone, Copy, Debug)]
pub(crate) struct TraceInst {
  pub pc: u32,
  pub inst: TInst,
}

/// 安装后逐入口校验面：live proto 的身份即「这些指令字 + 这些 k 常量位型」。
/// proto 被回收后地址复用的新 proto 只有在全部校验集逐位一致时才命中旧
/// trace——而校验集恰为 trace 的全部语义消费面，逐位一致即语义一致，
/// 校验通过本身构成正确性论证（无 ABA 窗口）。
pub(crate) struct TraceIdentity {
  pub sizecode: u32,
  /// FORNPREP..=FORNLOOP 全指令字（含头尾）
  pub words: Box<[u32]>,
  /// K 引用常量的 (k 下标, 位型) 平铺表（LOADN 立即数已含在 words 内）
  pub k_consts: Box<[(u32, u64)]>,
  /// GETIMPORT 缓存槽身份：(k 下标, (tt, value 8B) 合并位型)——import 缓存
  /// 被改写（不同 import 或失效）即身份失配重录
  pub import_ids: Box<[(u32, u128)]>,
}

/// 录制产物：闭体线性 IR + 回边 phi 集 + 类型特化分类。
pub(crate) struct TraceIr {
  /// 环出口（= fornloop_pc + 1，FORNPREP 零跳/环正常走完的续延位）
  pub exit_pc: u32,
  /// FORN 三元组首槽（limit=ra / step=ra+1 / idx=ra+2）
  pub ra: u8,
  pub insts: Vec<TraceInst>,
  /// 环不变 table 槽（GET/SET 表源；去重升序）
  pub tables: Vec<u8>,
  /// 环不变 number 槽（去重升序）
  pub inv_nums: Vec<u8>,
  /// 累加器 phi 槽（去重升序）
  pub accs: Vec<u8>,
  /// 区域写 acc 槽（分支区域内被更新的 acc——直写 d_acc，回边 fmov 豁免）
  pub region_accs: Vec<u8>,
  /// GETIMPORT 面：(dst 槽, k 下标)——prologue 常量拷贝（fast-path
  /// `setobj(ra, kv)` 的等价前置），入口守卫 safeenv + k[D] 非 nil 双守卫
  pub imports: Vec<(u8, u32)>,
  /// math 内建守卫槽（去重升序）——入口复检槽值仍为对应单参内建
  pub math_fn_guards: Vec<(u8, TMathUnary)>,
  /// 环体引用的 upvalue 分类表（去重；入口守卫形态/值面复检 + prologue
  /// 装载面）
  pub upvals: Vec<TUpval>,
  /// number 常量池 ((k 下标 | 占位, 值))
  pub consts: Vec<(u32, f64)>,
  /// 出口/bail 快照写回：(槽, 终值)。环体写过的全部槽按最后写定值——与
  /// 解释器「每迭代覆写、出口留终值」的可观测状态逐位对齐。
  pub writebacks: Vec<(u8, TVal)>,
  pub identity: TraceIdentity,
}

/// 单指令的 (读槽集, 写槽集)（位次预扫用；不支持的形态返回 None 拒录）。
/// K 形操作数无槽读；idx 槽读合法（phi）但不进分类集；limit/step 槽读拒录；
/// FORN 三元组槽写一律拒录。
fn inst_reads_writes(
  insn: Instruction,
  aux: Instruction,
  idx_slot: u8,
) -> Option<(Vec<u8>, Vec<u8>)> {
  let op = LuauOpcode::from(luau_insn_op(insn) as u8);
  let a = luau_insn_a(insn) as u8;
  let b = luau_insn_b(insn) as u8;
  let c = luau_insn_c(insn) as u8;
  let (rd, wr): (Vec<u8>, Vec<u8>) = match op {
    LuauOpcode::LOP_GETTABLE => (vec![b, c], vec![a]),
    LuauOpcode::LOP_GETTABLEN => (vec![b], vec![a]),
    LuauOpcode::LOP_SETTABLE => (vec![a, b, c], vec![]),
    LuauOpcode::LOP_SETTABLEN => (vec![a, b], vec![]),
    LuauOpcode::LOP_ADD | LuauOpcode::LOP_SUB | LuauOpcode::LOP_MUL | LuauOpcode::LOP_DIV => {
      (vec![b, c], vec![a])
    }
    LuauOpcode::LOP_MOD => (vec![b, c], vec![a]),
    LuauOpcode::LOP_ADDK | LuauOpcode::LOP_SUBK | LuauOpcode::LOP_MULK | LuauOpcode::LOP_DIVK => {
      (vec![b], vec![a])
    }
    LuauOpcode::LOP_MODK => (vec![b], vec![a]),
    LuauOpcode::LOP_MOVE => (vec![b], vec![a]),
    LuauOpcode::LOP_LOADN | LuauOpcode::LOP_LOADK => (vec![], vec![a]),
    LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => (vec![a], vec![]),
    LuauOpcode::LOP_JUMPXEQKN => (vec![a], vec![]),
    LuauOpcode::LOP_JUMPXEQKNIL => (vec![a], vec![]),
    LuauOpcode::LOP_GETIMPORT => (vec![], vec![a]),
    // CALL 特化面（math 单参内建族，B=2 参数+func、C=2 单返回；func 槽读
    // 由入口守卫消解，环内读面仅参数槽）：a=255 时 a+1 溢出槽域，拒录
    LuauOpcode::LOP_CALL => {
      if a == u8::MAX {
        return None;
      }
      (vec![a + 1], vec![a])
    }
    // FASTCALL1（bfid=A、参数=B、skip=C）：math 单参内建特化面（bfid 静态
    // 比对内建表）；返回槽 = 尾随 CALL 字的 A（走查有 code 访问，
    // prescan 面不建模——liveness 由走查 write_slot 收口）
    LuauOpcode::LOP_FASTCALL1 => (vec![b], vec![]),
    // GETUPVAL A B（T8）：upvalue 读不占栈槽读面（值经闭包 upref→cell 解析，
    // 非 number 载体由 Kind 分态拒承/守卫）；槽写同 MOVE（快照写回面收口）
    LuauOpcode::LOP_GETUPVAL => (vec![], vec![a]),
    // SETUPVAL A B（T8）：upvalue 写不占栈槽写面（写穿 cell，无快照回收面；
    // 效果前向单调）；值读面同算术操作数
    LuauOpcode::LOP_SETUPVAL => (vec![a], vec![]),
    LuauOpcode::LOP_JUMPIFEQ
    | LuauOpcode::LOP_JUMPIFLE
    | LuauOpcode::LOP_JUMPIFLT
    | LuauOpcode::LOP_JUMPIFNOTEQ
    | LuauOpcode::LOP_JUMPIFNOTLE
    | LuauOpcode::LOP_JUMPIFNOTLT => (vec![a, luau_insn_aux_a(aux) as u8], vec![]),
    _ => return None,
  };
  let step_slot = idx_slot.saturating_sub(1);
  let limit_slot = idx_slot.saturating_sub(2);
  // limit/step 槽读拒录（值语义未特化）；三元组槽写拒录
  if rd.contains(&limit_slot) || rd.contains(&step_slot) {
    return None;
  }
  if wr.iter().any(|&s| (limit_slot..=idx_slot).contains(&s)) {
    return None;
  }
  Some((rd, wr))
}

/// 指令字宽（比较跳转族双字：word1 = op|A<<8|D<<16，aux = B 寄存器号）。
fn inst_width(insn: Instruction) -> u32 {
  match LuauOpcode::from(luau_insn_op(insn) as u8) {
    LuauOpcode::LOP_JUMPIFEQ
    | LuauOpcode::LOP_JUMPIFLE
    | LuauOpcode::LOP_JUMPIFLT
    | LuauOpcode::LOP_JUMPIFNOTEQ
    | LuauOpcode::LOP_JUMPIFNOTLE
    | LuauOpcode::LOP_JUMPIFNOTLT
    | LuauOpcode::LOP_JUMPXEQKN
    | LuauOpcode::LOP_JUMPXEQKNIL
    | LuauOpcode::LOP_GETIMPORT => 2,
    _ => 1,
  }
}

/// 单指令的表位读槽（GET/SETTABLE(N) 的 B 槽——派生表资格分类用）。
fn inst_table_read(insn: Instruction) -> Option<u8> {
  match LuauOpcode::from(luau_insn_op(insn) as u8) {
    LuauOpcode::LOP_GETTABLE | LuauOpcode::LOP_GETTABLEN => Some(luau_insn_b(insn) as u8),
    LuauOpcode::LOP_SETTABLE => Some(luau_insn_b(insn) as u8),
    LuauOpcode::LOP_SETTABLEN => Some(luau_insn_b(insn) as u8),
    _ => None,
  }
}

/// 主走查状态（单遍；槽已知值只覆盖本迭代，跨回边可见性由预扫 phi 集保证）。
struct Walk<'k> {
  k: &'k [TValue],
  idx_slot: u8,
  accs: Vec<u8>,
  /// 区域写 acc 槽（分支区域内被更新——直写 d_acc 面）
  region_accs: Vec<u8>,
  acc_final: [Option<TVal>; 256],
  slot_value: [Option<TVal>; 256],
  written: Vec<u8>,
  tables: Vec<u8>,
  /// 派生表注册表（(src, index) 去重，id = 下标）
  derived: Vec<(TableRef, TVal)>,
  /// 槽 → 派生表 id（TableLoad 结果槽；该类槽值驻 x7，写回面排除）
  slot_tab: [Option<u8>; 256],
  /// CondLoad 结果槽（truthiness 面；写回面排除）
  cond_slots: Vec<u8>,
  /// generic 临时号集（CondLoad 产物；JUMPIF 真值判定面）
  generic_temps: Vec<u8>,
  /// GETIMPORT 收集面：(dst 槽, k 下标)——入口 prologue 常量拷贝 + 身份面
  imports: Vec<(u8, u32)>,
  /// math 单参内建 C 函数槽标记（预扫后按帧槽实测标定，槽 → 内建种）
  fn_slots: [Option<TMathUnary>; 256],
  /// upvalue 分类表（预扫构建；GETUPVAL/SETUPVAL 走查消费，inv_temp 走查期
  /// 惰性分配）
  upvals: Vec<TUpval>,
  /// 槽的表位/值位读面（预扫定案，GETTABLE 录制形态判据）
  table_read: [bool; 256],
  value_read: [bool; 256],
  /// 槽读次数（CondLoad 资格：单读且读点为紧邻分支）
  read_count: [u32; 256],
  inv_nums: Vec<u8>,
  temps: u8,
  consts: Vec<(u32, f64)>,
  insts: Vec<TraceInst>,
}

impl Walk<'_> {
  /// 槽读解析：idx → PhiIdx；累加器（未写到定义点）→ Acc 入参、定义点后 →
  /// 新值；本迭代已定值 → 该值；其余 → 环不变 number 引用。派生表槽
  /// （slot_tab）非 number 值，值位读即拒录。
  fn resolve(&mut self, slot: u8) -> Option<TVal> {
    if slot == self.idx_slot {
      return Some(TVal::PhiIdx);
    }
    if let Some(kind) = self.fn_slots[usize::from(slot)] {
      return Some(TVal::Cfn { slot, kind });
    }
    if self.slot_tab[slot as usize].is_some() {
      return None;
    }
    if let Some(i) = self.accs.iter().position(|&s| s == slot) {
      return match self.acc_final[slot as usize] {
        Some(v) => Some(v),
        None => Some(TVal::Acc(i as u8)),
      };
    }
    if let Some(v) = self.slot_value[slot as usize] {
      return Some(v);
    }
    match self.inv_nums.iter().position(|&s| s == slot) {
      Some(i) => Some(TVal::InvNum(i as u8)),
      None if self.inv_nums.len() < K_MAX_INV_NUMS => {
        self.inv_nums.push(slot);
        Some(TVal::InvNum((self.inv_nums.len() - 1) as u8))
      }
      None => None,
    }
  }

  /// 槽写登记：累加器单写 → 回边终值（多写即拒录，保守回解释器）；临时槽
  /// 允许覆写（T3 放宽：编译器跨指令复用临时寄存器是常态，链式算术形态
  /// 由此解锁——覆写的 SSA 语义 = 后续读点见最新写，与解释器「每迭代覆写、
  /// 后读见新值」逐位同构；`written` 只在首写时登记，供写回面按槽收口）。
  fn write_slot(&mut self, slot: u8, v: TVal) -> Option<()> {
    if self.accs.iter().position(|&s| s == slot).is_some() {
      // 区域写 acc 走 AccArith 专用面（直写 d_acc）；MOVE/LOADN/LOADK 落
      // 区域 acc 拒录（分支相关终值不可回边 fmov）
      if self.region_accs.contains(&slot) {
        return None;
      }
      if self.acc_final[slot as usize].is_some() {
        return None;
      }
      self.acc_final[slot as usize] = Some(v);
      return Some(());
    }
    if !self.written.contains(&slot) {
      self.written.push(slot);
    }
    self.slot_value[slot as usize] = Some(v);
    Some(())
  }

  /// 新临时编号（容量护栏）。
  fn fresh_temp(&mut self) -> Option<u8> {
    if self.temps >= K_MAX_TEMPS {
      return None;
    }
    let t = self.temps;
    self.temps += 1;
    Some(t)
  }

  /// 环不变 table 槽登记（本迭代已定值或累加器槽 → 拒录；类型面即 table）。
  /// 有派生表时收缩为 3 槽（x7/w13 让位派生表驻留）。
  fn intern_table(&mut self, slot: u8) -> Option<u8> {
    if self.slot_value[slot as usize].is_some() || self.accs.contains(&slot) {
      return None;
    }
    if let Some(i) = self.tables.iter().position(|&s| s == slot) {
      return Some(i as u8);
    }
    let cap = if self.derived.is_empty() {
      K_MAX_TABLES
    } else {
      K_MAX_TABLES - 1
    };
    if self.tables.len() >= cap {
      return None;
    }
    self.tables.push(slot);
    Some((self.tables.len() - 1) as u8)
  }

  /// 表源解析：槽持派生表 → Derived(id)；否则环不变槽登记。
  fn resolve_table(&mut self, slot: u8) -> Option<TableRef> {
    if let Some(d) = self.slot_tab[slot as usize] {
      return Some(TableRef::Derived(d));
    }
    self.intern_table(slot).map(TableRef::Inv)
  }

  /// 派生表登记（(src, index) 去重；x7/w13 单槽 + 环不变槽收缩面校验）。
  fn intern_derived(&mut self, src: TableRef, index: TVal) -> Option<u8> {
    if let Some(i) = self
      .derived
      .iter()
      .position(|&(s, ix)| s == src && ix == index)
    {
      return Some(i as u8);
    }
    if self.derived.len() >= K_MAX_DERIVED || self.tables.len() > K_MAX_TABLES - 1 {
      return None;
    }
    self.derived.push((src, index));
    Some((self.derived.len() - 1) as u8)
  }

  /// number 常量入池（同 k 下标去重——链式形态如 `i % 48 + (i + 7) % 48`
  /// 会让同一下标多次入池，不去重即虚耗池位触发容量拒录）。
  fn intern_const(&mut self, kidx: u32, v: f64) -> Option<usize> {
    if let Some(i) = self.consts.iter().position(|&(ki, _)| ki == kidx) {
      return Some(i);
    }
    if self.consts.len() >= K_MAX_CONSTS {
      return None;
    }
    self.consts.push((kidx, v));
    Some(self.consts.len() - 1)
  }

  /// k 下标取 number 常量（non-number 拒录）。
  fn k_number(&self, idx: u32) -> Option<f64> {
    let kv = self.k.get(idx as usize)?;
    if kv.tt != LuaType::Number as i32 {
      return None;
    }
    // Safety: tt==tnumber 保证 value 联合体 n 臂有效（k 常量池契约，
    // copatch const_number 同款读取）
    Some(unsafe { kv.value.n })
  }
}

/// FASTCALL1 A 字 bfid → math 单参内建种。prescan 影子录制与 walk 特化面
/// 共用——两侧必须同改（不对称：prescan 漏族 → 影子漏录落 Num 死轨静默
/// 复发；walk 漏族 → 整环拒录安全方向，故收口单点）。
fn fc1_math_unary(bfid: u8) -> Option<TMathUnary> {
  match LuauBuiltinFunction::from_id(i32::from(bfid)) {
    Some(LuauBuiltinFunction::LbfMathSqrt) => Some(TMathUnary::Sqrt),
    Some(LuauBuiltinFunction::LbfMathAbs) => Some(TMathUnary::Abs),
    Some(LuauBuiltinFunction::LbfMathFloor) => Some(TMathUnary::Floor),
    Some(LuauBuiltinFunction::LbfMathCeil) => Some(TMathUnary::Ceil),
    Some(LuauBuiltinFunction::LbfMathRound) => Some(TMathUnary::Round),
    _ => None,
  }
}

/// 录制 FORN 闭体 trace：结构性校验（闭体形态）+ 位次预扫（phi/不变分类）+
/// 环体走查（线性 IR）。
///
/// `fornprep_pc`/`fornloop_pc` 来自解释器现场（h_fornprep 传入的 FORNPREP 位点
/// 与 registry 按 d 域定位的 FORNLOOP），此处逐一复核闭环条件后静态走查。
/// `cl` = 当前活跃闭包（`ci->func`），upvalue 形态实测面消费。
///
/// # Safety
/// `code`/`k` 切片须派生自同一存活 proto（调用方 registry 契约），走查期只读；
/// `cl` 须为该帧活跃闭包，upref 读只触 tt/nupvalues。
pub(crate) unsafe fn record_forn_trace(
  code: &[Instruction],
  k: &[TValue],
  fornprep_pc: u32,
  fornloop_pc: u32,
  base: StkId,
  cl: *mut Closure,
) -> Option<TraceIr> {
  let fp = fornprep_pc as usize;
  let fl = fornloop_pc as usize;
  if fl <= fp || fl >= code.len() || fl - fp > K_MAX_TRACE_INSTS + 2 {
    return None;
  }
  // 头部形态：FORNPREP 且出口恰在 FORNLOOP 之后（闭体；fornprep_step 的
  // 不续延路 pc = fornprep_pc + 1 + d）
  let fp_insn = code[fp];
  if LuauOpcode::from(luau_insn_op(fp_insn) as u8) != LuauOpcode::LOP_FORNPREP {
    return None;
  }
  let ra = luau_insn_a(fp_insn) as u8;
  let exit = i64::from(fornprep_pc) + 1 + i64::from(luau_insn_d(fp_insn));
  if exit != fl as i64 + 1 || exit >= code.len() as i64 {
    return None;
  }
  // 尾部形态：FORNLOOP 且回边恰指环体头（h_fornloop 续延 npc = fl + 1 + d）
  let fl_insn = code[fl];
  if LuauOpcode::from(luau_insn_op(fl_insn) as u8) != LuauOpcode::LOP_FORNLOOP {
    return None;
  }
  if luau_insn_a(fl_insn) as u8 != ra {
    return None;
  }
  if i64::from(fornloop_pc) + 1 + i64::from(luau_insn_d(fl_insn)) != fp as i64 + 1 {
    return None;
  }

  let idx_slot = ra.saturating_add(2);

  // —— 位次预扫（pc 位点空间，双字比较跳转按宽步进）：读/写集合与首现
  // 位次 → 回边 phi 集；表位/值位读面 → 派生表资格；分支收集与区域规则 →
  // 菱形录制面（详见下方区域校验注）——
  let mut first_read = [usize::MAX; 256];
  let mut first_write = [usize::MAX; 256];
  let mut write_count = [0u8; 256];
  let mut table_read = [false; 256];
  let mut value_read = [false; 256];
  let mut read_count = [0u32; 256];
  let mut read_pcs: Vec<(u32, u8)> = Vec::new();
  let mut write_pcs: Vec<(u32, u8)> = Vec::new();
  let mut inst_pcs: Vec<u32> = Vec::new();
  // (pc, target, width)；target = pc + 1 + d（与 VJUMP 单式同源）
  let mut branches: Vec<(u32, u32, u32)> = Vec::new();
  // FASTCALL1 影子区 (fastcall pc, 尾随 CALL pc, bfid 内建种)：影子内
  // （fc_pc, call_pc] 的 GETUPVAL 为编译器内建追踪的回退面装载——热路被
  // FASTCALL1 跳过永不执行，特化种由 bfid 静态给出（func 槽录制期快照
  // 恒为上一轮结果数字，fn_slots 对此形态结构性失明）
  let mut fc_shadows: Vec<(u32, u32, TMathUnary)> = Vec::new();
  // (pc, upref 下标, 是否 SETUPVAL, 目标槽)——upvalue 分类面（T8）
  let mut upval_ops: Vec<(u32, u8, bool, u8)> = Vec::new();
  let mut pc = fp + 1;
  while pc < fl {
    let insn = code[pc];
    let width = inst_width(insn);
    let aux = if width == 2 && pc + 1 < fl {
      code[pc + 1]
    } else {
      insn
    };
    let (rd, wr) = inst_reads_writes(insn, aux, idx_slot)?;
    let trd = inst_table_read(insn);
    inst_pcs.push(pc as u32);
    let op = LuauOpcode::from(luau_insn_op(insn) as u8);
    match op {
      LuauOpcode::LOP_GETUPVAL => {
        upval_ops.push((
          pc as u32,
          luau_insn_b(insn) as u8,
          false,
          luau_insn_a(insn) as u8,
        ));
      }
      LuauOpcode::LOP_SETUPVAL => {
        upval_ops.push((
          pc as u32,
          luau_insn_b(insn) as u8,
          true,
          luau_insn_a(insn) as u8,
        ));
      }
      LuauOpcode::LOP_FASTCALL1 => {
        // 影子区仅 math 单参内建族（非本族 walk 整环拒录，无需影子）；
        // 尾随 CALL 界校验与 walk 臂同式
        if let Some(kind) = fc1_math_unary(luau_insn_a(insn) as u8) {
          let call_pc_i = pc as i64 + 1 + i64::from(luau_insn_c(insn));
          if call_pc_i > pc as i64 && call_pc_i < fl as i64 {
            fc_shadows.push((pc as u32, call_pc_i as u32, kind));
          }
        }
      }
      _ => {}
    }
    for r in rd {
      if r != idx_slot {
        if first_read[usize::from(r)] == usize::MAX {
          first_read[usize::from(r)] = pc;
        }
        read_count[usize::from(r)] += 1;
        read_pcs.push((pc as u32, r));
        if Some(r) == trd {
          continue; // 表位读不算值读（派生表资格分类面）
        }
        value_read[usize::from(r)] = true;
      }
    }
    if let Some(t) = trd {
      table_read[usize::from(t)] = true;
    }
    for w in wr {
      if first_write[usize::from(w)] == usize::MAX {
        first_write[usize::from(w)] = pc;
      }
      write_count[usize::from(w)] += 1;
      write_pcs.push((pc as u32, w));
    }
    if matches!(
      LuauOpcode::from(luau_insn_op(insn) as u8),
      LuauOpcode::LOP_JUMPIF
        | LuauOpcode::LOP_JUMPIFNOT
        | LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT
        | LuauOpcode::LOP_JUMPXEQKN
        | LuauOpcode::LOP_JUMPXEQKNIL
    ) {
      let target = (pc as i64) + 1 + i64::from(luau_insn_d(insn));
      if target <= pc as i64 || target > fl as i64 || pc + width as usize > fl {
        return None; // 后向/跨环出口/越体：跨 trace 跳转一律拒录
      }
      branches.push((pc as u32, target as u32, width));
    }
    pc += width as usize;
  }
  // —— 回边 phi 集（acc 分类）——
  let mut accs: Vec<u8> = Vec::new();
  for s in 0..=u8::MAX {
    if first_read[usize::from(s)] != usize::MAX
      && first_write[usize::from(s)] != usize::MAX
      && first_write[usize::from(s)] >= first_read[usize::from(s)]
    {
      if table_read[usize::from(s)] {
        // 表值跨回边（读点先于写点）不进 number phi 面，拒录
        return None;
      }
      if write_count[usize::from(s)] > 1 || accs.len() >= K_MAX_ACCS {
        return None;
      }
      accs.push(s);
    }
  }

  // —— math 单参内建 C 函数槽识别：body 引用槽中「is_c 闭包且 inner.c.f ==
  // math 库单参内建」者按内建种标 Cfn（CALL 特化消费面；槽跨环不变——体无
  // 写，入口守卫按槽复检）。槽域 = body 指令引用的读写槽并集，不越帧域。
  // 置于区域分析前：upvalue Cfn 载体判定与 GETUPVAL 虚写过滤都消费本集
  let mut fn_slots = [None; 256];
  {
    let mut ref_slots: Vec<u8> = Vec::new();
    for &(p, s) in read_pcs.iter().chain(write_pcs.iter()) {
      let _ = p;
      if !ref_slots.contains(&s) {
        ref_slots.push(s);
      }
    }
    for s in ref_slots {
      // Safety: body 指令引用槽为活跃帧槽域合法下标（派发环契约），
      // 只读 tt/gc 头与 inner.c.f
      unsafe {
        let tv = &*base.add(usize::from(s));
        if tv.is_function()
          // Safety: is_function() 谓词命中后 as_closure_ptr 为同址类型化读；
          // math_* 全仓单一定义，fn 项地址唯一（fn_address_comparisons 的
          // 「地址不保证唯一」保守提示不适用），转 usize 对账
          && let Some(kind) = slot_math_fn(tv)
        {
          fn_slots[usize::from(s)] = Some(kind);
        }
      }
    }
  }

  // —— upvalue 分类（T8）：体引用 upref 去重（首现序）；SETUPVAL 在场 =
  // 变更载体（逐访问穿 cell）；只读载体按 GETUPVAL 目标槽的 fn_slots 内容
  // 定 Kind（槽值 = cell 值——录制时点 ≥999 次迭代后，GETUPVAL 已把 cell
  // 值写入槽）。upref 形态（真 UpVal / LCT_VAL 值内联）按录制现场闭包
  // 实测锁定，prologue 生成码形态随之，入口守卫逐入口复核
  let mut upvals: Vec<TUpval> = Vec::new();
  for &(up_pc, b, is_set, a) in &upval_ops {
    // FASTCALL1 影子内 GETUPVAL = 编译器内建追踪的回退面装载（热路被
    // FASTCALL1 跳过，walk 亦跳过）——种由 bfid 静态给出；入口守卫按真值
    // 源复检，防环外重绑定后解释器走 CALL 回退而 trace 仍算旧内建的
    // 语义分叉。影子外维持 fn_slots 槽快照分类
    let fc_kind = fc_shadows
      .iter()
      .find(|&&(s, e, _)| up_pc > s && up_pc <= e)
      .map(|&(_, _, k)| k);
    if let Some(u) = upvals.iter_mut().find(|u| u.idx == b) {
      if is_set {
        u.mutated = true;
      } else if let Some(kind) = fc_kind.or(fn_slots[usize::from(a)]) {
        // 同 cell 同内容：两个只读 fn 槽必同种（跨种即录制现场损坏，拒录）
        if u.kind != TUpvalKind::Num && u.kind != TUpvalKind::Cfn(kind) {
          return None;
        }
        u.kind = TUpvalKind::Cfn(kind);
      }
    } else {
      if upvals.len() >= K_MAX_UPVALS {
        return None; // cell 指针驻留 x14/x15 两槽封顶
      }
      // Safety: cl 为 ci->func 活跃闭包（registry 契约），b < nupvalues 防御
      let cell = unsafe { upref_is_upval(cl, b) }?;
      // 只读载体首现即定 Kind（影子 Cfn / fn_slots Cfn）——若落 Num 而
      // 槽值实为函数载体，入口守卫 tnumber 会永拒承（死轨）；变更载体恒
      // Num（SETUPVAL 值面 number 特化，见下检查）
      let kind = match fc_kind.or(fn_slots[usize::from(a)]) {
        Some(kind) if !is_set => TUpvalKind::Cfn(kind),
        _ => TUpvalKind::Num,
      };
      upvals.push(TUpval {
        idx: b,
        cell,
        mutated: is_set,
        kind,
        inv_temp: None,
      });
    }
  }
  // 变更载体必为真 cell 且 number 种（SETUPVAL 值面 number 特化——LCT_VAL
  // 值内联 upref 不可变写，函数载体写值被 SETUPVAL 臂的 Cfn 拒面覆盖）
  if upvals
    .iter()
    .any(|u| u.mutated && (!u.cell || matches!(u.kind, TUpvalKind::Cfn(_))))
  {
    return None;
  }

  // —— Cfn 载体 GETUPVAL 的槽写为「虚写」（环内零代码，槽值由 CALL 臂写面
  // 承载，同解释器退出态）——从 write_pcs 剔除，防区域分析按「双无条件写」
  // 拒录 nbody sqrt 形态（GETUPVAL R_f + CALL R_f 同槽）
  if upvals
    .iter()
    .any(|u| matches!(u.kind, TUpvalKind::Cfn(_)) && !u.mutated)
  {
    let virtual_pcs: Vec<u32> = upval_ops
      .iter()
      .filter(|&&(_, b, is_set, _)| {
        !is_set
          && upvals
            .iter()
            .find(|u| u.idx == b)
            .is_some_and(|u| matches!(u.kind, TUpvalKind::Cfn(_)) && !u.mutated)
      })
      .map(|&(p, ..)| p)
      .collect();
    write_pcs.retain(|&(p, _)| !virtual_pcs.contains(&p));
  }

  // —— 区域校验（菱形录制面）：region = 分支落穿臂 [pc+width, target)——
  // 1) 分支目标必为已发射位点或环尾（continue 语义）；分支区间严格嵌套或
  //    相离（部分交叠 = 不可归约，拒录）；
  // 2) 区域内写槽限累加器（直写 d_acc 的区域 phi）或单写点、读点全含于
  //    最小包含区域的临时（区域死值）；区域内外混写 / 多区域写拒录
  //    （分支相关终值不可快照）；
  // 3) 区域写非 acc 槽入快照写回排除面（CondLoad 槽同理）。
  let mut region_accs: Vec<u8> = Vec::new();
  let mut no_writeback: Vec<u8> = Vec::new();
  for (i, &(_, btgt, _)) in branches.iter().enumerate() {
    if btgt != fl as u32 && !inst_pcs.contains(&btgt) {
      return None;
    }
    for &(b2, t2, _) in branches.iter().skip(i + 1) {
      if b2 < btgt && t2 > btgt {
        return None;
      }
    }
  }
  // pc 落入的区域集（含嵌套外层）
  let regions_of = |p: u32| -> Vec<(u32, u32)> {
    branches
      .iter()
      .filter_map(|&(b, t, w)| (p >= b + w && p < t).then_some((b + w, t)))
      .collect()
  };
  for slot in 0..=u8::MAX {
    let writes: Vec<u32> = write_pcs
      .iter()
      .filter(|&&(_, s)| s == slot)
      .map(|&(p, _)| p)
      .collect();
    if writes.is_empty() || writes.iter().all(|&p| regions_of(p).is_empty()) {
      continue; // 无写或全为无条件写
    }
    if accs.contains(&slot) {
      if !region_accs.contains(&slot) {
        region_accs.push(slot);
      }
      continue;
    }
    // 非 acc：单区域写点 → 读点全含于该区域（区域死值；出口/外推读即拒）。
    // 双写放行面：恰「先无条件写（区域外）+ 后区域写」——条件分支读点吃无
    // 条件值（发射面无条件覆盖 ✓），区域内读点吃寄存器值 ✓；该槽逐迭代被
    // 无条件重写 = 环内临时（跨环无活值），出口可见面排除写回即安全。其余
    //（≥3 写/双区域写/无条件写后置）终值随分支摇摆，拒录。
    if writes.len() == 2 {
      let (a, b) = (writes[0], writes[1]);
      let (ra_, rb_) = (regions_of(a), regions_of(b));
      let (uncond, region_w) = match (ra_.is_empty(), rb_.is_empty()) {
        (true, false) => (a, b),
        (false, true) => (b, a),
        _ => {
          return None; // 双区域写/同区双写：终值摇摆，拒录
        }
      };
      let Some(&(_rs, re)) = regions_of(region_w).iter().min_by_key(|&(_, e)| e) else {
        continue;
      };
      if uncond > region_w {
        return None; // 无条件写须先于区域写（分支读点吃无条件值的次序前提）
      }
      // 读点全在区域终前（分支读 + 区域内读）；出口/外推读即拒
      if !read_pcs.iter().all(|&(p, s2)| s2 != slot || p < re) {
        return None;
      }
      no_writeback.push(slot);
      continue;
    }
    if writes.len() > 1 {
      return None;
    }
    let Some(&(rs, re)) = regions_of(writes[0]).iter().min_by_key(|&(_, e)| e) else {
      continue;
    };
    // 读点全含于该区域（区域死值；出口/外推读即拒）
    if !read_pcs
      .iter()
      .all(|&(p, s)| s != slot || (p >= rs && p < re))
    {
      return None;
    }
    no_writeback.push(slot);
  }

  // —— 主走查 ——
  let mut w = Walk {
    k,
    idx_slot,
    accs,
    region_accs,
    acc_final: [None; 256],
    slot_value: [None; 256],
    written: Vec::new(),
    tables: Vec::new(),
    derived: Vec::new(),
    slot_tab: [None; 256],
    cond_slots: Vec::new(),
    generic_temps: Vec::new(),
    imports: Vec::new(),
    fn_slots,
    upvals,
    table_read,
    value_read,
    read_count,
    inv_nums: Vec::new(),
    temps: 0,
    consts: Vec::new(),
    insts: Vec::new(),
  };

  let mut pc = fp + 1;
  while pc < fl {
    let insn = code[pc];
    let op = LuauOpcode::from(luau_insn_op(insn) as u8);
    let (a, b, c) = (
      luau_insn_a(insn) as u8,
      luau_insn_b(insn) as u8,
      luau_insn_c(insn) as u8,
    );
    let inst = match op {
      // GETTABLE A B C：表源 B（环不变槽或派生表）、下标 C 任意 number
      // 操作数。结果槽作表用 → TableLoad 派生（嵌套形态 ts[r][j]）；作值用
      // → number 特化 ArrayLoad。双读共存拒录（值读会坠 number 特化 bail
      // 风暴）。
      LuauOpcode::LOP_GETTABLE => {
        let index = w.resolve(c)?;
        if w.table_read[usize::from(a)] {
          if w.value_read[usize::from(a)]
            || w.accs.contains(&a)
            || w.slot_tab[usize::from(a)].is_some()
          {
            return None;
          }
          let src = w.resolve_table(b)?;
          let derived = w.intern_derived(src, index)?;
          w.slot_tab[usize::from(a)] = Some(derived);
          TInst::TableLoad {
            slot: a,
            src,
            index,
          }
        } else {
          let table = w.resolve_table(b)?;
          let dst = w.fresh_temp()?;
          // 布尔载资格：单读 + 读点为紧邻 JUMPIF(NOT)（tag 驻 W9 跨发射点
          // 存活）；acc 目标拒录（入口 number tag 守卫与 truthiness 面矛盾）
          let generic = pc + 1 < fl
            && w.read_count[usize::from(a)] == 1
            && matches!(
              LuauOpcode::from(luau_insn_op(code[pc + 1]) as u8),
              LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT
            )
            && luau_insn_a(code[pc + 1]) as u8 == a
            && !w.accs.contains(&a);
          w.write_slot(a, TVal::Temp(dst))?;
          if generic {
            w.generic_temps.push(dst);
            w.cond_slots.push(a);
            TInst::CondLoad {
              dst,
              slot: a,
              table,
              index,
            }
          } else {
            TInst::ArrayLoad { dst, table, index }
          }
        }
      }
      // GETTABLEN A B C：0 基立即数下标；结果作表用 → TableLoad（常量下标
      // 入池占位），作值用 → number 特化
      LuauOpcode::LOP_GETTABLEN => {
        let t = w.resolve_table(b)?;
        if w.table_read[usize::from(a)] {
          if w.value_read[usize::from(a)]
            || w.accs.contains(&a)
            || w.slot_tab[usize::from(a)].is_some()
          {
            return None;
          }
          let cid = w.intern_const(K_NINDEX_PLACEHOLDER, f64::from(c) + 1.0)?;
          let derived = w.intern_derived(t, TVal::Const(cid))?;
          w.slot_tab[usize::from(a)] = Some(derived);
          TInst::TableLoad {
            slot: a,
            src: t,
            index: TVal::Const(cid),
          }
        } else {
          let dst = w.fresh_temp()?;
          // 布尔载资格（同 GETTABLE 臂注）
          let generic = pc + 1 < fl
            && w.read_count[usize::from(a)] == 1
            && matches!(
              LuauOpcode::from(luau_insn_op(code[pc + 1]) as u8),
              LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT
            )
            && luau_insn_a(code[pc + 1]) as u8 == a
            && !w.accs.contains(&a);
          w.write_slot(a, TVal::Temp(dst))?;
          if generic {
            w.generic_temps.push(dst);
            w.cond_slots.push(a);
            TInst::CondLoadN {
              dst,
              slot: a,
              table: t,
              c,
            }
          } else {
            TInst::ArrayLoadN { dst, table: t, c }
          }
        }
      }
      // SETTABLE A B C：`table[index] = value`（A=值，B=表，C=下标任意
      // number 操作数；派生表存储穿透的 readonly 守卫在 TableLoad 位点逐
      // 迭代收口）
      LuauOpcode::LOP_SETTABLE => {
        let t = w.resolve_table(b)?;
        let index = w.resolve(c)?;
        let value = w.resolve(a)?;
        TInst::ArrayStore {
          table: t,
          index,
          value,
        }
      }
      LuauOpcode::LOP_SETTABLEN => {
        let t = w.resolve_table(b)?;
        let value = w.resolve(a)?;
        TInst::ArrayStoreN { table: t, c, value }
      }
      // 算术族：寄存器双操作数
      LuauOpcode::LOP_ADD => arith_walk(&mut w, a, b, c, TArith::Add)?,
      LuauOpcode::LOP_SUB => arith_walk(&mut w, a, b, c, TArith::Sub)?,
      LuauOpcode::LOP_MUL => arith_walk(&mut w, a, b, c, TArith::Mul)?,
      LuauOpcode::LOP_DIV => arith_walk(&mut w, a, b, c, TArith::Div)?,
      LuauOpcode::LOP_MOD => arith_walk(&mut w, a, b, c, TArith::Mod)?,
      // K 形算术：C 字 = k 下标，须 number 常量
      LuauOpcode::LOP_ADDK | LuauOpcode::LOP_SUBK | LuauOpcode::LOP_MULK | LuauOpcode::LOP_DIVK => {
        // C 字 = 8 位 k 下标（u32 域，无负值面）
        let kc = luau_insn_c(insn);
        let cv = w.k_number(kc)?;
        let cid = w.intern_const(kc, cv)?;
        let lhs = w.resolve(b)?;
        let op = match op {
          LuauOpcode::LOP_ADDK => TArith::Add,
          LuauOpcode::LOP_SUBK => TArith::Sub,
          LuauOpcode::LOP_MULK => TArith::Mul,
          _ => TArith::Div,
        };
        arith_walk_val(&mut w, a, lhs, TVal::Const(cid), op)?
      }
      LuauOpcode::LOP_MODK => {
        let kc = luau_insn_c(insn);
        let cv = w.k_number(kc)?;
        let cid = w.intern_const(kc, cv)?;
        let lhs = w.resolve(b)?;
        arith_walk_val(&mut w, a, lhs, TVal::Const(cid), TArith::Mod)?
      }
      // MOVE A B：槽写（终值快照落 writebacks，环内零代码）
      LuauOpcode::LOP_MOVE => {
        let v = w.resolve(b)?;
        w.write_slot(a, v)?;
        pc += 1;
        continue;
      }
      // LOADN A Bx：number 立即数落槽（16 位 Bx：b|c<<8，copatch 同式）
      LuauOpcode::LOP_LOADN => {
        let imm = (luau_insn_b(insn) | (luau_insn_c(insn) << 8)) as i32;
        let cid = w.intern_const(K_LOADN_PLACEHOLDER, f64::from(imm))?;
        w.write_slot(a, TVal::Const(cid))?;
        pc += 1;
        continue;
      }
      // LOADK A D：number 常量落槽（非 number 拒录）
      LuauOpcode::LOP_LOADK => {
        let kd = luau_insn_b(insn) | (luau_insn_c(insn) << 8);
        let cv = w.k_number(kd)?;
        let cid = w.intern_const(kd, cv)?;
        w.write_slot(a, TVal::Const(cid))?;
        pc += 1;
        continue;
      }
      // FASTCALL1 A B C 特化：A=bfid（静态比对 math 单参内建表）、B=参数槽、
      // C=skip——尾随 CALL 字载返回槽（A）与返回数（C-1=1），语义位对齐
      // dispatch_fastcall 的对应内建（快路成功：写 ra、pc += skip+1；本族
      // 内建不在 LUAU_F_TABLE 时解释器落慢路 full call，语义同为
      // check_number + 单返回写槽，特化面一致）。参数/返回皆 number 特化面
      //（同 CALL 版论证）
      LuauOpcode::LOP_FASTCALL1 => {
        let Some(kind) = fc1_math_unary(a) else {
          return None; // 仅 math 单参内建族特化
        };
        let call_pc_i = pc as i64 + 1 + i64::from(c);
        if call_pc_i <= pc as i64 || call_pc_i >= fl as i64 {
          return None; // 尾随 CALL 字须在环体内
        }
        let call_pc = call_pc_i as usize;
        if LuauOpcode::from(luau_insn_op(code[call_pc]) as u8) != LuauOpcode::LOP_CALL {
          return None;
        }
        let dst_slot = luau_insn_a(code[call_pc]) as u8;
        if i64::from(luau_insn_c(code[call_pc])) - 1 != 1 {
          return None; // 单返回形态
        }
        let arg = w.resolve(b)?;
        if is_generic(&w, arg) || matches!(arg, TVal::Cfn { .. }) {
          return None;
        }
        let dst = w.fresh_temp()?;
        w.write_slot(dst_slot, TVal::Temp(dst))?;
        w.insts.push(TraceInst {
          pc: pc as u32,
          inst: TInst::MathUnary {
            dst,
            dst_slot,
            guard: None,
            kind,
            arg,
          },
        });
        pc = call_pc + 1; // 跳过尾随 CALL 字（其执行已被 fast call 消化）
        continue;
      }
      // CALL A B C 特化：B=2（func+单参）C=2（单返回）且 func 槽为 math
      // 单参内建 C 函数槽 → 单指令直译（参数/返回皆 number 特化面，非
      // number 参数在解释器同执行路径必然先抛 check_number——特化环的
      // 参数面皆 number 已知值，无守卫面）。func 槽读由入口守卫消解（
      // 内建槽跨环不变，体无写），环内读面仅参数槽
      LuauOpcode::LOP_CALL => {
        let b_count = luau_insn_b(insn);
        let c_count = luau_insn_c(insn);
        let TVal::Cfn {
          slot: src_slot,
          kind,
        } = w.resolve(a)?
        else {
          return None; // 非 math 内建调用形态整环拒录
        };
        if b_count != 2 || c_count != 2 {
          return None;
        }
        let arg = w.resolve(a + 1)?;
        if is_generic(&w, arg) || matches!(arg, TVal::Cfn { .. }) {
          return None;
        }
        let dst = w.fresh_temp()?;
        w.write_slot(a, TVal::Temp(dst))?;
        TInst::MathUnary {
          dst,
          dst_slot: a,
          guard: Some(src_slot),
          kind,
          arg,
        }
      }
      // GETUPVAL A B（T8 解锁面）：upvalue 读三分态——
      // - 变更载体（体含 SETUPVAL）：逐访问穿 cell 指针装载（UpvalLoad，tag
      //   守卫 bail）——跨迭代读写经 cell 传递，同迭代读后写见新值；
      // - 只读 Num 载体：prologue 内联保留临时（入口守卫钉 cell 源 tnumber，
      //   环内态不可变由无调用约束保证），环内零代码，槽写走快照写回面；
      // - 只读 Cfn 载体（nbody `sqrt(dist2)` 形态）：环内零代码，值经
      //   fn_slots 身份被 CALL 特化消费；槽写为虚写（prescan 已剔除，CALL
      //   臂写面承载，同解释器退出态）
      LuauOpcode::LOP_GETUPVAL => {
        let ui = w.upvals.iter().position(|u| u.idx == b)?;
        if w.upvals[ui].mutated {
          let dst = w.fresh_temp()?;
          w.write_slot(a, TVal::Temp(dst))?;
          TInst::UpvalLoad { dst, uv: ui as u8 }
        } else {
          match w.upvals[ui].kind {
            TUpvalKind::Cfn(_) => {
              pc += 1;
              continue;
            }
            TUpvalKind::Num => {
              let t = match w.upvals[ui].inv_temp {
                Some(t) => t,
                None => w.fresh_temp()?,
              };
              w.upvals[ui].inv_temp = Some(t);
              w.write_slot(a, TVal::Temp(t))?;
              pc += 1;
              continue;
            }
          }
        }
      }
      // SETUPVAL A B（T8 解锁面）：upvalue 写穿 cell（值 + tnumber tag 双写，
      // open 态栈穿透 / closed 态 storage 同一指针语义；屏障对 number 构造性
      // 零动作）。值面 number 特化：generic/Cfn 值拒录（解释器承接任意类型
      // 写——写一个布尔/表进 upvalue 的环不走本面）
      LuauOpcode::LOP_SETUPVAL => {
        let ui = w.upvals.iter().position(|u| u.idx == b)?;
        let value = w.resolve(a)?;
        if is_generic(&w, value) || matches!(value, TVal::Cfn { .. }) {
          return None;
        }
        TInst::UpvalStore {
          uv: ui as u8,
          value,
        }
      }
      // GETIMPORT A D（双字，aux 为链参数）：import 缓存常量拷贝。安全
      // 双守卫（safeenv + k[D] 非 nil）提至入口（环体无调用无赋值，缓存
      // 与 safeenv 环不变）；发射 = prologue 全 TValue 拷贝（k[D]→槽 A，
      // 位对齐解释器 fast-path `setobj(ra, kv)`），环内零代码；dst 槽不进
      // walk 已知值（表位/数值/真值各读面按槽自然路径解析）；k[D] 位型
      // 进身份面（import 缓存被改写即重录）
      LuauOpcode::LOP_GETIMPORT => {
        let kidx = luau_insn_d(insn) as u32;
        if kidx > 255 {
          return None; // kidx*16 须在寻址 imm12 域（≤ 4080）
        }
        w.imports.push((a, kidx));
        let width = inst_width(insn);
        pc += width as usize;
        continue;
      }
      // —— 条件跳转族（菱形双径直译）——
      // JUMPIF/JUMPIFNOT A D：truthiness。条件为 number 已知值（算术临时/
      // 常量/不变槽/acc/idx）→ 常量折叠（number 恒真：JUMPIF 折无条件跳、
      // JUMPIFNOT 抹除）；紧邻 CondLoad 的 generic 临时 → 真值判定面。
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        let positive = op == LuauOpcode::LOP_JUMPIF;
        let target = (i64::from(pc as u32) + 1 + i64::from(luau_insn_d(insn))) as u32;
        let cond = match w.resolve(a)? {
          TVal::Temp(t) if w.generic_temps.contains(&t) => TCond::Truth { temp: t, positive },
          _ if positive => TCond::Always,
          // number 恒真 → JUMPIFNOT 永不跳，零发射
          _ => {
            pc += 1;
            continue;
          }
        };
        TInst::Branch { cond, target }
      }
      // JUMPXEQKNIL：nil 常量比较（hit = is_nil(ra)，not 位反转）——tag 面
      // 独立判定（GETIMPORT/任意类型槽的真值组合面），不进 number 比较
      LuauOpcode::LOP_JUMPXEQKNIL => {
        let aux = code[pc + 1];
        let target = (i64::from(pc as u32) + 1 + i64::from(luau_insn_d(insn))) as u32;
        TInst::Branch {
          cond: TCond::Nil {
            slot: a,
            positive: luau_insn_aux_not(aux) == 0,
          },
          target,
        }
      }
      // 比较跳转族（双字）：word1 = op|A<<8|D<<16，aux = B 寄存器号。
      // 双操作数须 number 已知值（generic 临时拒录）。
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT
      | LuauOpcode::LOP_JUMPXEQKN => {
        let aux = code[pc + 1];
        let positive = !matches!(
          op,
          LuauOpcode::LOP_JUMPIFNOTEQ | LuauOpcode::LOP_JUMPIFNOTLE | LuauOpcode::LOP_JUMPIFNOTLT
        );
        let cmp = match op {
          LuauOpcode::LOP_JUMPIFEQ | LuauOpcode::LOP_JUMPIFNOTEQ | LuauOpcode::LOP_JUMPXEQKN => {
            TCmpOp::Eq
          }
          LuauOpcode::LOP_JUMPIFLE | LuauOpcode::LOP_JUMPIFNOTLE => TCmpOp::Le,
          _ => TCmpOp::Lt,
        };
        let target = (i64::from(pc as u32) + 1 + i64::from(luau_insn_d(insn))) as u32;
        let lhs = w.resolve(a)?;
        let rhs = if op == LuauOpcode::LOP_JUMPXEQKN {
          // K 常量比较（hit = is_number && val == k；操作数面皆 number 已知值，
          // tag 面同构）。k 下标 = aux 低 24 位（not 位在 bit 31，单列提取）
          let kidx = luau_insn_aux_kv(aux);
          let kv = w.k_number(kidx)?;
          let cid = w.intern_const(kidx, kv)?;
          TVal::Const(cid)
        } else {
          w.resolve(luau_insn_aux_a(aux) as u8)?
        };
        let positive = if op == LuauOpcode::LOP_JUMPXEQKN {
          luau_insn_aux_not(aux) == 0
        } else {
          positive
        };
        if is_generic(&w, lhs) || is_generic(&w, rhs) {
          return None; // generic 值不进数值比较面
        }
        TInst::Branch {
          cond: TCond::Cmp {
            op: cmp,
            lhs,
            rhs,
            positive,
          },
          target,
        }
      }
      _ => return None,
    };
    let width = inst_width(insn);
    w.insts.push(TraceInst {
      pc: pc as u32,
      inst,
    });
    pc += width as usize;
  }

  // 快照写回面：全部环内写过的槽（含累加器 phi 槽——其写点走 acc_final
  // 早退分支，不入 written，须并集收口），终值 = 累加器回边值或已知值
  // （累加器必有唯一定义点，acc_final 已在写点收口）。派生表槽排除——
  // 槽值由 TableLoad 逐迭代回写保持新鲜，快照重写反而可能覆盖（bail 在
  // TableLoad 前）。CondLoad 槽与区域写非 acc 槽同理排除。
  let mut writebacks: Vec<(u8, TVal)> = w
    .accs
    .iter()
    .filter_map(|&s| Some((s, w.acc_final[s as usize]?)))
    .collect();
  for &s in &w.written {
    if w.accs.contains(&s)
      || w.slot_tab[usize::from(s)].is_some()
      || w.cond_slots.contains(&s)
      || no_writeback.contains(&s)
    {
      continue;
    }
    writebacks.push((s, w.slot_value[s as usize]?));
  }

  // 校验面：头尾 + 环体全指令字 + K 引用常量位型平铺表
  let words: Box<[u32]> = code[fp..=fl].to_vec().into_boxed_slice();
  let k_consts: Box<[(u32, u64)]> = w
    .consts
    .iter()
    .filter(|&&(kidx, _)| kidx < K_NINDEX_PLACEHOLDER)
    .map(|&(kidx, v)| (kidx, v.to_bits()))
    .collect::<Vec<_>>()
    .into_boxed_slice();

  // math 内建守卫槽收集（去重）：入口复检「内建槽仍为对应 math C 函数」——
  // 跨入口该局部可能被重赋值，特化环须按槽拒承
  let mut math_fn_guards: Vec<(u8, TMathUnary)> = w
    .insts
    .iter()
    .filter_map(|ti| match ti.inst {
      TInst::MathUnary {
        guard: Some(src_slot),
        kind,
        ..
      } => Some((src_slot, kind)),
      _ => None,
    })
    .collect();
  math_fn_guards.sort_unstable();
  math_fn_guards.dedup();

  // GETIMPORT 身份面：(tt, value 8B) 合并位型——import 缓存位型对账
  let mut import_ids: Vec<(u32, u128)> = w
    .imports
    .iter()
    .map(|&(_, kidx)| {
      let kv = &k[kidx as usize];
      let bits = ((kv.tt as u128) << 64) | ((unsafe { kv.value.n } as u64) as u128);
      (kidx, bits)
    })
    .collect();
  import_ids.sort_unstable();
  import_ids.dedup();

  Some(TraceIr {
    exit_pc: exit as u32,
    ra,
    imports: w.imports,
    math_fn_guards,
    upvals: w.upvals,
    insts: w.insts,
    tables: w.tables,
    inv_nums: w.inv_nums,
    accs: w.accs,
    region_accs: w.region_accs,
    consts: w.consts,
    writebacks,
    identity: TraceIdentity {
      sizecode: code.len() as u32,
      words,
      k_consts,
      import_ids: import_ids.into_boxed_slice(),
    },
  })
}

/// generic 临时判定（CondLoad 产物；不进数值比较面）。
fn is_generic(w: &Walk<'_>, v: TVal) -> bool {
  matches!(v, TVal::Temp(t) if w.generic_temps.contains(&t))
}

/// upref 形态实测：`cl.uprefs[b]` 是否持真 UpVal（ttisupval）。false = LCT_VAL
/// 值内联 upref（克隆时常量 TValue）。形态由 proto 捕获描述符决定、跨入口
/// 稳定（同 proto 全闭包同形态），入口守卫仍逐入口复核。
///
/// # Safety
/// `cl` 须为活跃闭包（registry 契约 `ci->func`）；`b` 为字节码 B 字，越界
/// nupvalues 返回 None（防御面，调用方拒录）。
unsafe fn upref_is_upval(cl: *mut Closure, b: u8) -> Option<bool> {
  // Safety: 契约见函数注；uprefs 为柔性数组语义，指针算术寻址（VM_UV 同式）
  unsafe {
    if b >= (*cl).nupvalues {
      return None;
    }
    let ur = addr_of!((*cl).inner.l.uprefs).cast::<TValue>();
    Some((*ur.add(usize::from(b))).is_upval())
  }
}

/// 回边武装资格预扫（T2 回边计数）：FORNPREP 头形态成立且环体全部指令落在
/// 支持集（无调用/分配/元方法族）即判定「有录制可能」。只做廉价静态
/// 面，不承责任何录制成功性——主走查期的容量/单写纪律失败仍会拒录。消费方
/// 是 registry 的回边计数武装决策：不可录形态的环（体内含 CALL 等）不武装，
/// 解释器回边零计数税。
pub(crate) fn forn_body_eligible(code: &[Instruction], fornprep_pc: u32, fornloop_pc: u32) -> bool {
  let fp = fornprep_pc as usize;
  let fl = fornloop_pc as usize;
  if fl <= fp || fl >= code.len() {
    return false;
  }
  let fp_insn = code[fp];
  if LuauOpcode::from(luau_insn_op(fp_insn) as u8) != LuauOpcode::LOP_FORNPREP {
    return false;
  }
  let idx_slot = (luau_insn_a(fp_insn) as u8).saturating_add(2);
  let mut pc = fp + 1;
  while pc < fl {
    let insn = code[pc];
    let width = inst_width(insn);
    let aux = if width == 2 && pc + 1 < fl {
      code[pc + 1]
    } else {
      insn
    };
    if inst_reads_writes(insn, aux, idx_slot).is_none() {
      return false;
    }
    pc += width as usize;
  }
  true
}

/// 算术指令走查（寄存器双操作数）。
fn arith_walk(w: &mut Walk<'_>, a: u8, b: u8, c: u8, op: TArith) -> Option<TInst> {
  let lhs = w.resolve(b)?;
  let rhs = w.resolve(c)?;
  arith_walk_val(w, a, lhs, rhs, op)
}

/// 算术指令走查（第二操作数已解析形态）。区域写 acc（分支区域内被更新的
/// phi 槽）走 AccArith 专用面——直写 d_acc，跳转未走即自然保持；acc_final
/// 登记 Acc(i) 自指（writebacks 面对区域 acc 写 d_acc 即当前值，出口/bail
/// 两态皆真——漏登记会让 writebacks 过滤丢槽，exit 丢累计）。
/// Cfn 操作数拒录（函数值进算术位在解释器是抛错面，特化环不得以 D_MOD
/// 防御寄存器静默错算——T8 起 GETUPVAL 解锁后 fn 槽暴露面变宽，一并封口）。
fn arith_walk_val(w: &mut Walk<'_>, a: u8, lhs: TVal, rhs: TVal, op: TArith) -> Option<TInst> {
  if matches!(lhs, TVal::Cfn { .. }) || matches!(rhs, TVal::Cfn { .. }) {
    return None;
  }
  if w.region_accs.contains(&a) {
    let acc = w.accs.iter().position(|&s| s == a)? as u8;
    if w.acc_final[usize::from(a)].is_some() {
      return None;
    }
    w.acc_final[usize::from(a)] = Some(TVal::Acc(acc));
    return Some(TInst::AccArith { acc, op, lhs, rhs });
  }
  let dst = w.fresh_temp()?;
  w.write_slot(a, TVal::Temp(dst))?;
  Some(TInst::Arith { dst, op, lhs, rhs })
}
