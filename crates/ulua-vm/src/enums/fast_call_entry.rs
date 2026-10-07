//! `LUAU_F_TABLE` 表项查找的三态视图（§11 路线 pass B FASTCALL 系列的收敛载体）。
//!
//! cpp 侧查表是裸函数指针三态（lvmexecute.cpp:3050 `luau_FastFunction f =
//! LUAU_F_TABLE[bfid]; LUAU_ASSERT(f);`）：表无此项、空槽（函数指针为 NULL，即
//! `LBF_NONE`/blocked 态，上游新版的 `luauF_blocked` 同义）、可调用项。本仓槽位
//! 存储为 [`LuauFastFunction`]（`Option<fn>`）配定长 256 表，「无此项」由索引
//! 边界表达（`.get()` 越界 → `None`），「空槽」由槽内 `None` 表达——两级 Option
//! 折叠进 [`FastCallEntry::from`] 后，调用点收敛为一个 `match`，不再出现
//! `is_some()` 判定 + `let Some(f) = f else` 解包链，也不引入任何空指针哨兵。
//!
//! 与 [`ValueView`](crate::enums::value_view::ValueView) 同属"零 ABI 风险"边界：
//! 表的内存布局（JIT 经 `offset_of!` 直接按槽位 `blr` 调用）、`build_table`
//! 静态初始化与一切 `luau_f_*` 处理器签名均不变，仅改解释器读侧的分支形状。
use crate::type_aliases::luau_fast_function::{LuauFastCallable, LuauFastFunction};

/// 三态表项：见模块文档。payload 直接携带可调用 fn 指针（[`LuauFastCallable`]
/// 为 [`LuauFastFunction`] 的 `Some` 分支同型别名），匹配成功后调用点无需再解包。
#[derive(Clone, Copy, Debug)]
pub enum FastCallEntry {
  /// 表无此项（下标越界）。当前字节码 `bfid` 为 8 位、表长 256，实际不可达，
  /// 但该态让构造侧可以吃下 `Option<LuauFastFunction>` 而无需前置边界判定。
  Absent,
  /// 表项存在但函数指针为空槽（`LBF_NONE`，cpp 的 blocked/禁用态；快速调用
  /// 不得进入，回退慢路径）。
  Blocked,
  /// 表项存在且可调用。
  Fast(LuauFastCallable),
}

/// 自当前查表结果（`.get(bfid).copied()` 的 `Option<LuauFastFunction>` 两级
/// Option 形状）构造三态视图。
impl From<Option<LuauFastFunction>> for FastCallEntry {
  fn from(lookup: Option<LuauFastFunction>) -> Self {
    match lookup {
      Some(Some(f)) => Self::Fast(f),
      Some(None) => Self::Blocked,
      None => Self::Absent,
    }
  }
}
