//! Inline port of `luau/tests/VecDeque.test.cpp` (`TEST_SUITE("VecDequeTests")`).
//! Tests Luau's custom ring-buffer `VecDeque`: exact capacity growth, front/back
//! queues, random access (`at`/index), contiguity, shrink-to-fit, and clone.
//!
//! Adaptations to Rust: C++ distinguishes copy-construction (preserves the raw
//! buffer layout) from copy-assignment (which may normalize to contiguous). Rust
//! has a single `Clone` that preserves the layout (see the impl), so both clones
//! stay discontiguous — these tests assert Rust's actual clone behavior. C++
//! move (which empties the source) becomes a Rust move (the source is consumed,
//! so post-move source checks are inapplicable and omitted).

use std::rc::Rc;

use ulua_common::records::vec_deque::VecDeque;

const SSO: [&str; 10] = [
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];
const LONG: [&str; 10] = [
  "Love doesn't just sit there, like a stone, it has to be made, like bread; remade all the time, made new.",
  "People who deny the existence of dragons are often eaten by dragons. From within.",
  "It is good to have an end to journey toward; but it is the journey that matters, in the end.",
  "We're each of us alone, to be sure. What can you do but hold your hand out in the dark?",
  "When you light a candle, you also cast a shadow.",
  "You cannot buy the revolution. You cannot make the revolution. You can only be the revolution. It is in your spirit, or it is nowhere.",
  "To learn which questions are unanswerable, and not to answer them: this skill is most needful in times of stress and darkness.",
  "What sane person could live in this world and not be crazy?",
  "The only thing that makes life possible is permanent, intolerable uncertainty: not knowing what comes next.",
  "My imagination makes me human and makes me a fool; it gives me all the world and exiles me from it.",
];

fn string_sets() -> [[String; 10]; 2] {
  [SSO.map(String::from), LONG.map(String::from)]
}

// ---- int queues ----

#[test]
fn forward_queue_test_no_initial_capacity() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  assert!(queue.empty());
  for i in 0..10 {
    queue.push_back(i);
  }
  assert!(!queue.empty());
  assert_eq!(queue.size(), 10);
  assert_eq!(queue.capacity(), 11);
  for j in 0..10 {
    assert_eq!(*queue.front(), j);
    assert_eq!(*queue.back(), 9);
    assert!(!queue.empty());
    queue.pop_front();
  }
}

#[test]
fn forward_queue_test() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  queue.reserve(5);
  assert!(queue.empty());
  for i in 0..10 {
    queue.push_back(i);
  }
  assert_eq!(queue.size(), 10);
  assert_eq!(queue.capacity(), 13);
  for j in 0..10 {
    assert_eq!(*queue.front(), j);
    assert_eq!(*queue.back(), 9);
    queue.pop_front();
  }
}

#[test]
fn forward_queue_test_initializer_list() {
  let mut queue = VecDeque::from_init_list(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
  assert!(!queue.empty());
  assert_eq!(queue.size(), 10);
  assert_eq!(queue.capacity(), 10);
  for j in 0..10 {
    assert_eq!(*queue.front(), j);
    assert_eq!(*queue.back(), 9);
    queue.pop_front();
  }
}

#[test]
fn reverse_queue_test() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  queue.reserve(5);
  for i in 0..10 {
    queue.push_front(i);
  }
  assert_eq!(queue.size(), 10);
  assert_eq!(queue.capacity(), 13);
  for j in 0..10 {
    assert_eq!(*queue.front(), 9);
    assert_eq!(*queue.back(), j);
    queue.pop_back();
  }
}

#[test]
fn random_access_queue_test() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  queue.reserve(5);
  for i in 0..10 {
    queue.push_back(i);
  }
  assert_eq!(queue.size(), 10);
  for j in 0..10usize {
    assert_eq!(*queue.at(j), j as i32);
    assert_eq!(*queue.operator_index(j), j as i32);
  }
}

#[test]
fn clear_works_on_queue() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  queue.reserve(5);
  for i in 0..10 {
    queue.push_back(i);
  }
  assert_eq!(queue.size(), 10);
  for j in 0..10usize {
    assert_eq!(*queue.operator_index(j), j as i32);
  }
  queue.clear();
  assert!(queue.empty());
  assert_eq!(queue.size(), 0);
}

#[test]
fn pop_front_at_end() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  queue.reserve(5);
  queue.push_front(0);
  for i in 1..10 {
    queue.push_back(i);
  }
  assert_eq!(queue.size(), 10);
  for j in 0..10 {
    assert_eq!(*queue.front(), j);
    assert_eq!(*queue.back(), 9);
    queue.pop_front();
  }
}

#[test]
fn pop_back_at_front() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  queue.reserve(5);
  queue.push_back(0);
  for i in 1..10 {
    queue.push_front(i);
  }
  assert_eq!(queue.size(), 10);
  for j in 0..10 {
    assert_eq!(*queue.front(), 9);
    assert_eq!(*queue.back(), j);
    queue.pop_back();
  }
}

#[test]
fn queue_is_contiguous() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  for i in 0..10 {
    queue.push_back(i);
  }
  assert_eq!(queue.size(), 10);
  assert_eq!(queue.capacity(), 11);
  assert!(queue.is_contiguous());
}

#[test]
fn queue_is_not_contiguous() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  for i in 5..10 {
    queue.push_back(i);
  }
  for i in (0..5).rev() {
    queue.push_front(i);
  }
  assert_eq!(queue.size(), 10);
  assert_eq!(queue.capacity(), 11);
  assert!(!queue.is_contiguous());
  for j in 0..10usize {
    assert_eq!(*queue.operator_index(j), j as i32);
  }
}

#[test]
fn shrink_to_fit_works() {
  let mut queue: VecDeque<i32> = VecDeque::new();
  for i in 5..10 {
    queue.push_back(i);
  }
  for i in (0..5).rev() {
    queue.push_front(i);
  }
  assert_eq!(queue.size(), 10);
  assert_eq!(queue.capacity(), 11);
  assert!(!queue.is_contiguous());
  for j in 0..10usize {
    assert_eq!(*queue.operator_index(j), j as i32);
  }
  queue.shrink_to_fit();
  assert!(queue.is_contiguous());
  assert_eq!(queue.capacity(), queue.size());
  for j in 0..10usize {
    assert_eq!(*queue.operator_index(j), j as i32);
  }
}

// ---- string queues (run over both the SSO and long-string sets) ----

#[test]
fn string_queue_test_no_initial_capacity() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    for item in ts.iter().take(10) {
      queue.push_back(item.clone());
    }
    assert_eq!(queue.size(), 10);
    assert_eq!(queue.capacity(), 11);
    for item in ts.iter().take(10) {
      assert_eq!(queue.front(), item);
      assert_eq!(*queue.back(), ts[9]);
      queue.pop_front();
    }
  }
}

#[test]
fn string_queue_test() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.reserve(5);
    for item in ts.iter().take(10) {
      queue.push_back(item.clone());
    }
    assert_eq!(queue.size(), 10);
    assert_eq!(queue.capacity(), 13);
    for item in ts.iter().take(10) {
      assert_eq!(queue.front(), item);
      assert_eq!(*queue.back(), ts[9]);
      queue.pop_front();
    }
  }
}

#[test]
fn string_queue_test_initializer_list() {
  for ts in string_sets() {
    let mut queue = VecDeque::from_init_list(ts.to_vec());
    assert_eq!(queue.size(), 10);
    assert_eq!(queue.capacity(), 10);
    for j in 0..10 {
      assert_eq!(*queue.front(), ts[j]);
      assert_eq!(*queue.back(), ts[9]);
      queue.pop_front();
    }
  }
}

#[test]
fn reverse_string_queue_test() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.reserve(5);
    for item in ts.iter().take(10) {
      queue.push_front(item.clone());
    }
    assert_eq!(queue.size(), 10);
    assert_eq!(queue.capacity(), 13);
    for item in ts.iter().take(10) {
      assert_eq!(*queue.front(), ts[9]);
      assert_eq!(queue.back(), item);
      queue.pop_back();
    }
  }
}

#[test]
fn random_access_string_queue_test() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.reserve(5);
    for item in ts.iter().take(10) {
      queue.push_back(item.clone());
    }
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue.at(j), item);
      assert_eq!(queue.operator_index(j), item);
    }
  }
}

#[test]
fn clear_works_on_string_queue() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.reserve(5);
    for item in ts.iter().take(10) {
      queue.push_back(item.clone());
    }
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue.operator_index(j), item);
    }
    queue.clear();
    assert!(queue.empty());
    assert_eq!(queue.size(), 0);
  }
}

#[test]
fn pop_front_string_at_end() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.reserve(5);
    queue.push_front(ts[0].clone());
    for item in ts[1..10].iter() {
      queue.push_back(item.clone());
    }
    assert_eq!(queue.size(), 10);
    for item in ts.iter().take(10) {
      assert_eq!(queue.front(), item);
      assert_eq!(*queue.back(), ts[9]);
      queue.pop_front();
    }
  }
}

#[test]
fn pop_back_string_at_front() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.reserve(5);
    queue.push_back(ts[0].clone());
    for item in ts[1..10].iter() {
      queue.push_front(item.clone());
    }
    assert_eq!(queue.size(), 10);
    for item in ts.iter().take(10) {
      assert_eq!(*queue.front(), ts[9]);
      assert_eq!(queue.back(), item);
      queue.pop_back();
    }
  }
}

#[test]
fn string_queue_is_contiguous() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    for item in ts.iter().take(10) {
      queue.push_back(item.clone());
    }
    assert_eq!(queue.size(), 10);
    assert_eq!(queue.capacity(), 11);
    assert!(queue.is_contiguous());
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue.operator_index(j), item);
    }

    // Clone preserves layout + capacity (C++ copy construction).
    let queue2 = queue.clone();
    assert_eq!(queue2.size(), 10);
    assert_eq!(queue2.capacity(), 11);
    assert!(queue2.is_contiguous());
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue2.operator_index(j), item);
    }

    // Move (C++ move construction); source is consumed.
    let queue4 = queue2;
    assert_eq!(queue4.size(), 10);
    assert_eq!(queue4.capacity(), 11);
    assert!(queue4.is_contiguous());
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue4.operator_index(j), item);
    }
  }
}

#[test]
fn string_queue_is_not_contiguous() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    for item in ts[5..10].iter() {
      queue.push_back(item.clone());
    }
    for item in ts[..5].iter().rev() {
      queue.push_front(item.clone());
    }
    assert_eq!(queue.size(), 10);
    assert_eq!(queue.capacity(), 11);
    assert!(!queue.is_contiguous());
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue.operator_index(j), item);
    }

    // Rust clone preserves the discontiguous layout (unlike C++ copy-
    // assignment, which normalizes — Rust has only one Clone).
    let queue2 = queue.clone();
    assert!(!queue2.is_contiguous());
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue2.operator_index(j), item);
    }

    // Move from discontiguous, then grow — must stay correct.
    let mut queue4 = queue;
    assert!(!queue4.is_contiguous());
    queue4.push_back(String::from("zero"));
    queue4.push_back(String::from("?"));
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue4.operator_index(j), item);
    }
    assert_eq!(*queue4.operator_index(10), "zero");
    assert_eq!(*queue4.operator_index(11), "?");

    // Reserve from discontiguous — must stay correct.
    let mut queue5 = queue2;
    queue5.reserve(20);
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue5.operator_index(j), item);
    }
  }
}

#[test]
fn shrink_to_fit_works_with_strings() {
  for ts in string_sets() {
    let mut queue: VecDeque<String> = VecDeque::new();
    for item in ts[5..10].iter() {
      queue.push_back(item.clone());
    }
    for item in ts[..5].iter().rev() {
      queue.push_front(item.clone());
    }
    assert_eq!(queue.size(), 10);
    assert_eq!(queue.capacity(), 11);
    assert!(!queue.is_contiguous());
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue.operator_index(j), item);
    }
    queue.shrink_to_fit();
    assert!(queue.is_contiguous());
    assert_eq!(queue.capacity(), queue.size());
    for (j, item) in ts.iter().enumerate().take(10) {
      assert_eq!(queue.operator_index(j), item);
    }
  }
}

struct TestStruct;

#[test]
fn push_front_elements_are_destroyed_correctly() {
  let t = Rc::new(TestStruct);
  {
    let mut queue: VecDeque<Rc<TestStruct>> = VecDeque::new();
    queue.reserve(10);
    queue.push_front(t.clone());
    queue.push_front(t.clone());
    assert_eq!(Rc::strong_count(&t), 3);

    let _queue2 = queue.clone();
    let _queue3 = queue.clone();
    // queue, _queue2, _queue3 all dropped at scope end.
  }
  assert_eq!(Rc::strong_count(&t), 1);
}

// ---- iteration (iter / iter_mut / into_iter) ----

#[test]
fn iter_yields_logical_order() {
  // 无环绕：单段连续。
  let queue = VecDeque::from_init_list(vec![0, 1, 2, 3, 4]);
  let collected: Vec<i32> = queue.iter().copied().collect();
  assert_eq!(collected, vec![0, 1, 2, 3, 4]);
  assert_eq!(queue.iter().len(), 5);
  assert_eq!(queue.iter().count(), 5);
}

#[test]
fn iter_yields_logical_order_wrapped() {
  // 环绕：pop_front 推进 head 后 push_back 回绕到缓冲区开头。
  let mut queue = VecDeque::from_init_list(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
  queue.pop_front();
  queue.pop_front();
  queue.push_back(10);
  queue.push_back(11);
  assert!(!queue.is_contiguous());
  let collected: Vec<i32> = queue.iter().copied().collect();
  assert_eq!(collected, vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
  // 双端迭代：从 back 反向遍历。
  let reversed: Vec<i32> = queue.iter().rev().copied().collect();
  assert_eq!(reversed, vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2]);
  // next_back 与 next 交错。
  let mut it = queue.iter();
  assert_eq!(it.next(), Some(&2));
  assert_eq!(it.next_back(), Some(&11));
  assert_eq!(it.len(), 8);
  // 空容器：迭代器为空。
  assert!(VecDeque::<i32>::new().iter().next().is_none());
}

#[test]
fn iter_mut_modifies_elements_in_place() {
  let mut queue = VecDeque::from_init_list(vec![1, 2, 3]);
  queue.pop_front();
  queue.push_back(4);
  // 环绕状态下原地修改。
  for v in queue.iter_mut() {
    *v *= 10;
  }
  let collected: Vec<i32> = queue.iter().copied().collect();
  assert_eq!(collected, vec![20, 30, 40]);
}

#[test]
fn into_iter_consumes_in_logical_order_and_drops_rest() {
  // 完整消费：逻辑顺序出队。
  let mut queue = VecDeque::from_init_list(vec![0, 1, 2, 3]);
  queue.pop_front();
  queue.push_back(4);
  let collected: Vec<i32> = queue.into_iter().collect();
  assert_eq!(collected, vec![1, 2, 3, 4]);

  // 双端消费 + 未消费元素随迭代器析构（Rc 引用计数回落）。
  let dropped = Rc::new(TestStruct);
  {
    let mut queue: VecDeque<Rc<TestStruct>> = VecDeque::new();
    queue.reserve(4);
    for _ in 0..3 {
      queue.push_back(dropped.clone());
    }
    assert_eq!(Rc::strong_count(&dropped), 4);
    let mut it = queue.into_iter();
    assert!(it.next().is_some());
    assert!(it.next_back().is_some());
    assert_eq!(it.len(), 1);
    // it 在此 drop：剩余 1 个元素随迭代器析构。
  }
  assert_eq!(Rc::strong_count(&dropped), 1);

  // 空容器 into_iter：无元素。
  assert!(VecDeque::<i32>::new().into_iter().next().is_none());
}

#[test]
fn into_iterator_impls_match_std_semantics() {
  // for-in 循环走 &VecDeque 的 IntoIterator。
  let mut queue = VecDeque::from_init_list(vec![5, 6, 7]);
  let mut seen = Vec::new();
  for v in &queue {
    seen.push(*v);
  }
  assert_eq!(seen, vec![5, 6, 7]);
  // &mut VecDeque 的 IntoIterator。
  for v in &mut queue {
    *v += 1;
  }
  assert_eq!(*queue.front(), 6);
}
