use alloc::string::String;

use crate::{
  functions::{dot_escape::dot_escape, to_string_constraint_graph::to_string_constraint_vertex},
  records::constraint_graph::ConstraintGraph,
};

impl ConstraintGraph {
  pub fn dump(&mut self) {
    for (v, deps) in self.dependencies.iter() {
      let vstr = unsafe { to_string_constraint_vertex(v.clone()) };
      let deps_ref = unsafe { &**deps };
      for d in deps_ref.order.iter() {
        // The C++ `for (auto d : *deps)` iterates only present entries.
        if !deps_ref.contains(d.clone()) {
          continue;
        }

        let mut line = String::new();
        dot_escape(&mut line, &vstr);
        line.push_str(" -> ");
        let dstr = unsafe { to_string_constraint_vertex(d.clone()) };
        dot_escape(&mut line, &dstr);
        std::println!("{}", line);
      }
    }
  }
}
