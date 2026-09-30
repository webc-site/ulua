//! Arena 身份值化：`Type.owning_arena` / `TypePackVar.owning_arena` 由 cpp
//! 自引用身份指针 `TypeArena* owningArena`（`Type.h:877` / `TypePack.h:132`）
//! 迁移为本 crate 的纯值类型 [`ArenaId`]。
//!
//! 换算约定：cpp `ty->owningArena == arena`（指针相等）↔ Rust
//! `ty.owning_arena == arena.arena_id`（值相等）。**不设 id→arena 注册表**：
//! 全部归属比较的对照侧都是调用点手里已存活的 arena（引用、`Handle` 或
//! 内嵌字段），取其 `arena_id` 即可；比较退化为 `u32` 值相等，节点侧解引用
//! 读出的不再是裸地址而是按值 Copy 的身份，原「仅比指针、不解引用 arena」
//! 一族的 SAFETY 证成命题随类型消失。

use core::{
  fmt::{self, Debug, Formatter},
  num::NonZeroU32,
  sync::atomic::{AtomicU32, Ordering},
};

/// arena 的进程级唯一身份标识（`TypeArena::arena_id` 的字段类型）。
///
/// 内部以 [`NonZeroU32`] 存储、逻辑值偏置 1：`value()` 为对外语义的
/// arena 编号，`NONE.value() == 0` 复刻 cpp `nullptr` 哨兵（「无归属
/// arena」），构造期未接线与日志脱链节点均持有 [`ArenaId::NONE`]。
/// 真实 arena 从逻辑编号 1 起经 [`next_arena_id`] 单调发号，进程内
/// 永不复用，故值相等 ⇔ 同一 arena，与原指针相等判据逐位等价。
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArenaId(NonZeroU32);

impl ArenaId {
  /// 「无归属 arena」哨兵，对应 cpp `nullptr`：存储值 1（偏置位）、`value()` 为 0。
  /// 发号器从存储值 2 起分配，该值不会作为真实 arena 身份出现。
  pub const NONE: Self = Self(NonZeroU32::MIN);

  /// 对外语义的 arena 编号；[`ArenaId::NONE`] 为 0（nullptr 哨兵），
  /// 真实 arena 自 1 起。
  #[inline]
  pub const fn value(self) -> u32 {
    // 存储值恒 ≥1（NonZeroU32），偏置减法不回绕。
    self.0.get() - 1
  }
}

impl Debug for ArenaId {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if *self == Self::NONE {
      f.write_str("ArenaId::NONE")
    } else {
      write!(f, "ArenaId({})", self.value())
    }
  }
}

/// 全进程单调发号：`TypeArena` 构造时领取自身身份，此后不可变。
///
/// relaxed 序即可：发号只要求进程内互异，不发布任何跨线程可见性。
/// 存储值从 2 起（1 为 [`ArenaId::NONE`] 保留）；u32 计数需 40 亿次
/// arena 构造才回绕，属不可触发边界。
pub(crate) fn next_arena_id() -> ArenaId {
  static NEXT_STORED: AtomicU32 = AtomicU32::new(2);
  let stored = NEXT_STORED.fetch_add(1, Ordering::Relaxed);
  ArenaId(NonZeroU32::new(stored).expect("arena 发号计数器回绕（进程生命周期内不可触发）"))
}
