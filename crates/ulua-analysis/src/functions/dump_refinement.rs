extern crate alloc;

use alloc::string::String;

use crate::{
  functions::dump_def::dump_def, type_aliases::refinement_control_flow_graph::Refinement,
};

pub fn dump_refinement(r: &Refinement) -> String {
  match r {
    Refinement::Proposition(p) => {
      let lhs = unsafe { dump_def(p.ptr) };
      if let Some(ty) = &p.r#type {
        let guard = if p.is_typeof { "typeof" } else { "type" };
        let cmp = if p.sense { "==" } else { "~=" };
        format!("{} {} {} \"{}\"", lhs, guard, cmp, ty)
      } else {
        format!("{}{}", lhs, if p.sense { " truthy" } else { " falsy" })
      }
    }
    Refinement::Conjunction(c) => unsafe {
      format!(
        "({} && {})",
        dump_refinement(&*c.lhs),
        dump_refinement(&*c.rhs)
      )
    },
    Refinement::Disjunction(d) => unsafe {
      format!(
        "({} || {})",
        dump_refinement(&*d.lhs),
        dump_refinement(&*d.rhs)
      )
    },
    Refinement::Negation(n) => unsafe { format!("!({})", dump_refinement(&*n.refinement)) },
  }
}
