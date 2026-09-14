use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::tarjan_result::TarjanResult,
  records::{tarjan::Tarjan, tarjan_worklist_vertex::TarjanWorklistVertex},
};
impl Tarjan {
  pub fn loop_item(&mut self) -> TarjanResult {
    while !self.worklist.is_empty() {
      let (index, mut curr_edge, mut last_edge) = {
        let top = self.worklist.last().unwrap();
        (top.index, top.curr_edge, top.last_edge)
      };

      // First visit
      if curr_edge == -1 {
        self.child_count += 1;
        if self.child_limit > 0 && self.child_limit <= self.child_count {
          return TarjanResult::TooManyChildren;
        }

        self.stack.push(index);

        self.nodes[index as usize].on_stack = true;

        curr_edge = self.edges_ty.len() as i32;

        // Fill in edge list of this vertex
        let ty = self.nodes[index as usize].ty;
        if !ty.is_null() {
          self.visit_children_type_id_i32(ty, index);
        } else {
          let tp = self.nodes[index as usize].tp;
          if !tp.is_null() {
            self.visit_children_type_pack_id_i32(tp, index);
          }
        }

        last_edge = self.edges_ty.len() as i32;

        // Persist updated curr/last edge values back into worklist entry
        if let Some(top) = self.worklist.last_mut() {
          top.curr_edge = curr_edge;
          top.last_edge = last_edge;
        }
      }

      // Visit children
      let mut found_fresh = false;
      while curr_edge < last_edge {
        let mut child_index: i32 = -1;
        let mut fresh = false;

        let edge_ty = self.edges_ty[curr_edge as usize];
        if !edge_ty.is_null() {
          let (ci, fr) = self.indexify_type_id(edge_ty);
          child_index = ci;
          fresh = fr;
        } else {
          let edge_tp = self.edges_tp[curr_edge as usize];
          if !edge_tp.is_null() {
            let (ci, fr) = self.indexify_type_pack_id(edge_tp);
            child_index = ci;
            fresh = fr;
          } else {
            LUAU_ASSERT!(false);
          }
        }

        if fresh {
          // Original recursion point, update the parent continuation point and start the new element
          if let Some(top) = self.worklist.last_mut() {
            top.curr_edge = curr_edge + 1;
            // top.last_edge unchanged
          }
          self.worklist.push(TarjanWorklistVertex {
            index: child_index,
            curr_edge: -1,
            last_edge: -1,
          });
          found_fresh = true;
          break;
        } else if self.nodes[child_index as usize].on_stack {
          let ll = self.nodes[index as usize].lowlink;
          let other = child_index;
          if other < ll {
            self.nodes[index as usize].lowlink = other;
          }
        }

        self.visit_edge(child_index, index);

        curr_edge += 1;
      }

      if found_fresh {
        continue;
      }

      if self.nodes[index as usize].lowlink == index {
        self.visit_scc(index);

        while !self.stack.is_empty() {
          let popped = *self.stack.last().unwrap();
          self.stack.pop();

          self.nodes[popped as usize].on_stack = false;
          if popped == index {
            break;
          }
        }
      }

      self.worklist.pop();

      // Original return from recursion into a child
      if !self.worklist.is_empty() {
        let (parent_index, _parent_curr_edge, parent_end_edge) = {
          let top = self.worklist.last().unwrap();
          (top.index, top.curr_edge, top.last_edge)
        };

        // No need to keep child edges around
        let new_len = parent_end_edge as usize;
        self.edges_ty.truncate(new_len);
        self.edges_tp.truncate(new_len);

        let child_lowlink = self.nodes[index as usize].lowlink;
        let parent_lowlink_ref = &mut self.nodes[parent_index as usize].lowlink;
        if child_lowlink < *parent_lowlink_ref {
          *parent_lowlink_ref = child_lowlink;
        }

        self.visit_edge(index, parent_index);
      }
    }

    TarjanResult::Ok
  }
}
