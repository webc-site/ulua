use alloc::string::String;

use crate::{
  functions::{dot_escape::dot_escape, to_string_constraint_graph::to_string_constraint_vertex},
  records::constraint_graph::ConstraintGraph,
};

impl ConstraintGraph {
  pub fn dump(&mut self) {
    for (v, deps) in self.dependencies.iter() {
      // Safety: to_string_constraint_vertex 为 unsafe fn（清单外），要求 vertex 各
      // 变体句柄可解引用——dependencies 的键全部经 add_dependency_of_* 插入：
      // TypeId/TypePackId 指向会话类型 arena 存活节点（bump 块不移动），
      // *const Constraint 指向 self.constraint_lists（PinnedStorage 的 Box 内容，
      // 地址稳定）；dump 是只读调试遍历，不失效任何句柄。
      let vstr = unsafe { to_string_constraint_vertex(v.clone()) };
      // Safety: deps 值由 find_dependency_list 从 self.constraint_lists 分配后写入
      // 映射，非空且指向存活 ConstraintList（Box 内容地址稳定，后续 push 不移动）；
      // 迭代期间映射未写，共享借用与上方键借用分属不同对象，可并存。
      let deps_ref = unsafe { &**deps };
      for d in deps_ref.order.iter() {
        // The C++ `for (auto d : *deps)` iterates only present entries.
        if !deps_ref.contains(d.clone()) {
          continue;
        }

        let mut line = String::new();
        dot_escape(&mut line, &vstr);
        line.push_str(" -> ");
        // Safety: d 是 deps 列表中登记的顶点句柄，与键 v 同源（arena 存活类型/
        // pack/ConstraintList 内 Constraint），只读格式化不改变存活性。
        let dstr = unsafe { to_string_constraint_vertex(d.clone()) };
        dot_escape(&mut line, &dstr);
        std::println!("{}", line);
      }
    }
  }
}
