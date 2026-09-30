extern crate alloc;

use alloc::string::String;

use crate::{
  functions::dump_def::dump_def, type_aliases::refinement_control_flow_graph::Refinement,
};

pub fn dump_refinement(r: &Refinement) -> String {
  match r {
    Refinement::Proposition(p) => {
      // `p.ptr` 是 `DefId = SymDefId` u32 句柄（#17）——由 ControlFlowGraph
      // 构建期经 `register_sym_def` 发放；`dump_def` 经注册表解析读
      // `versioned_name`，未命中（含空哨兵）输出 "?"（cpp nullptr 分支同效），
      // 全程只读、safe。
      let lhs = dump_def(p.ptr);
      if let Some(ty) = &p.r#type {
        let guard = if p.is_typeof { "typeof" } else { "type" };
        let cmp = if p.sense { "==" } else { "~=" };
        format!("{}({}) {} \"{}\"", guard, lhs, cmp, ty)
      } else {
        format!("{}{}", lhs, if p.sense { " truthy" } else { " falsy" })
      }
    }
    Refinement::Conjunction(c) => {
      // `c.lhs`/`c.rhs` 为 NotNull `RefinementId = Handle<Refinement>`，即构建期
      // 接线、指向 CFG 自有 arena 中存活子细化节点的句柄；`get()` 物化共享
      // 只读借用（契约收口在 arena_handle.rs），递归只读、单线程无并存别名。
      format!(
        "({} && {})",
        dump_refinement(c.lhs.get()),
        dump_refinement(c.rhs.get())
      )
    }
    Refinement::Disjunction(d) => {
      // 同 Conjunction——`d.lhs`/`d.rhs` 是构建期接线的 NotNull `RefinementId`
      // arena 句柄，递归只读。
      format!(
        "({} || {})",
        dump_refinement(d.lhs.get()),
        dump_refinement(d.rhs.get())
      )
    }
    Refinement::Negation(n) => {
      // `n.refinement` 同为 NotNull `RefinementId` arena 句柄；递归仅只读该
      // 子节点。
      format!("!({})", dump_refinement(n.refinement.get()))
    }
  }
}
