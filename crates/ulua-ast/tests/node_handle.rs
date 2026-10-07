//! `records::node_handle` 公开面测试(review.md §8:测公开 API 的测试放 crate 外
//! `tests/`)。构造端用普通堆/栈对象 + [`Node::from_mut`]/[`Node::from_ref`]
//! 即满足 arena 契约的可测替身:句柄数组只检验「元素身份透传 + 空态安全」,
//! 与被指对象的 AST 身份无关。

use core::ptr::eq;

use ulua_ast::records::node_handle::{Node, Nodes, OptNode};

#[derive(Debug)]
struct Marker(u32);
// 注:字段仅用于区分断言消息,不被读取。

fn marker_nodes(markers: &mut [Marker]) -> Nodes<Marker> {
  Nodes::from_vec(markers.iter_mut().map(Node::from_mut).collect())
}

mod element_iter {
  use core::ptr::from_mut;

  use super::*;

  #[test]
  fn nodes_iter_mut_writes_through_and_is_double_ended() {
    let mut markers = [Marker(0), Marker(1), Marker(2)];
    let mut owned = marker_nodes(&mut markers);
    assert_eq!(owned.iter_mut().len(), 3);
    for m in owned.iter_mut() {
      m.0 += 10;
    }
    assert_eq!(
      markers.iter().map(|m| m.0).collect::<Vec<_>>(),
      [10, 11, 12]
    );
    // 只读面同步可见
    assert_eq!(owned.iter().map(|m| m.0).collect::<Vec<_>>(), [10, 11, 12]);

    let mut owned = marker_nodes(&mut markers);
    let mut it = owned.iter_mut();
    assert!(from_mut(it.next_back().unwrap()) == from_mut(&mut markers[2]));
    assert!(from_mut(it.next().unwrap()) == from_mut(&mut markers[0]));
    it.next().unwrap().0 = 99; // 中段元素 = markers[1]
    assert!(it.next().is_none());
    assert_eq!(markers[1].0, 99);
  }

  #[test]
  fn nodes_as_mut_slice_and_into_iter_mut() {
    let mut markers = [Marker(0)];
    let mut owned = marker_nodes(&mut markers);
    owned.as_mut_slice()[0].0 = 7;
    assert_eq!(markers[0].0, 7);
    for m in &mut owned {
      m.0 += 1;
    }
    assert_eq!(markers[0].0, 8);
    assert_eq!((&owned).into_iter().count(), 1);
  }
}

mod opt_node {
  use core::ptr::from_mut;

  use super::*;

  #[test]
  fn map_extracts_value_from_wired_slot() {
    let mut marker = Marker(7);
    let slot = OptNode::from_ptr(from_mut(&mut marker));
    assert_eq!(slot.map(|m| m.0), Some(7));
    // 映射结果可借用节点字段(挂在 &self 上,非 Copy 消耗)
    let borrowed: Option<&u32> = slot.map(|m| &m.0);
    assert!(eq(borrowed.unwrap(), &marker.0));
  }

  #[test]
  fn map_on_empty_slot_is_none() {
    let empty: OptNode<Marker> = OptNode::default();
    assert!(empty.is_none());
    assert_eq!(empty.map(|m| m.0), None);
  }

  #[test]
  fn unwrap_or_prefers_wired_slot_then_falls_back() {
    let mut wired = Marker(1);
    let fallback = Marker(0);
    let slot = OptNode::from_ptr(from_mut(&mut wired));
    assert!(eq(slot.unwrap_or(&fallback), &wired));

    let empty: OptNode<Marker> = OptNode::default();
    assert!(eq(empty.unwrap_or(&fallback), &fallback));
  }
}

mod temp_vector_transfer {
  use core::ptr::from_mut;

  use ulua_ast::records::temp_vector::TempVector;

  use super::*;

  #[test]
  fn from_temp_vector_copies_slots_and_scratch_is_released() {
    let mut markers = [Marker(0), Marker(1)];
    let mut scratch: Vec<*mut Marker> = Vec::new();
    let nodes;
    {
      let mut tv = TempVector::new(&mut scratch);
      tv.push_back(from_mut(&mut markers[0]));
      tv.push_back(from_mut(&mut markers[1]));
      nodes = Nodes::from_temp_vector(&tv);
      assert_eq!(nodes.len(), 2);
      assert!(eq(nodes.get(0).unwrap(), &markers[0]));
      assert!(eq(nodes.get(1).unwrap(), &markers[1]));
    } // TempVector 的 Drop 把 scratch 截回空窗
    assert!(scratch.is_empty());
    // 句柄数组独立堆持有，scratch 回收后仍可读（cpp copy 的转移语义）
    assert_eq!(nodes.iter().map(|m| m.0).collect::<Vec<_>>(), [0, 1]);
  }

  #[test]
  fn empty_temp_vector_degrades_per_flavor() {
    let mut scratch: Vec<*mut Marker> = Vec::new();
    let tv = TempVector::<*mut Marker>::new(&mut scratch);
    assert!(Nodes::from_temp_vector(&tv).is_empty());
  }
}
