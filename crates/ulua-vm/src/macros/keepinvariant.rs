use crate::{
  macros::{
    gc_satomic::GCSATOMIC,
    gc_spropagate::{GCSPROPAGATE, GCSPROPAGATEAGAIN},
  },
  records::global_state::global_State,
};

/// GC 增量期是否须维持「黑不指白」不变式（cpp `lgc.h` `keepinvariant`）。
///
/// B 档契约前移（参照 `abs_index`/`isyielded` 先例）：原 `*const global_State`
/// 存活契约改由 `&` 接收者的引用有效性规则在调用点承载；被调体只读一个普通
/// `gcstate` 字段（非 union 成员），全 safe。
#[inline(always)]
pub(crate) const fn keepinvariant(g: &global_State) -> bool {
  let gcstate = g.gcstate as i32;
  matches!(gcstate, GCSPROPAGATE | GCSPROPAGATEAGAIN | GCSATOMIC)
}
