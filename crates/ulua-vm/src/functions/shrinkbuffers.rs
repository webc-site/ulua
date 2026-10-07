use crate::{
  functions::stringresizeprotected::stringresizeprotected, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——r12-w6 收形：`l` 前移 `&mut LuaState` 引用形，签名不再
/// 携带调用方裸指针；体内窄块只解引用 LuaState 不变量保护的自有 `global` 字段及其串表，
/// `stringresizeprotected` 的裸 `l` 转手自独占借用就地重建、借用窗止于当句，按 r16-v21 判例
/// 保留，故本体降为安全 `fn`）：`l` 须处于可分配/可 rehash 的受保护帧（收缩经 OOM 可抛错）。
/// 负载判定与减半目标见 `Stringtable::wants_shrink/half_size`。
pub(crate) fn shrinkbuffers(l: &mut LuaState) {
  // SAFETY: 块内 `g` 为 `l` 自有不变量保护的存活 `global_State` 裸指针；`l` 转手由独占
  // 借用承载，受保护帧前提见本函数契约。
  unsafe {
    let g = l.global;
    if (*g).strt.wants_shrink() {
      let half = (*g).strt.half_size();
      stringresizeprotected(l.as_mut_ptr(), half);
    }
  }
}
