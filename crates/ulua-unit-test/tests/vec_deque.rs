//! Port of `cpp/tests/VecDeque.test.cpp`（889 行，26 个 TEST_CASE）。
//!
//! 被测对象：`ulua_common::records::vec_deque::VecDeque`。
//! 对应 C++ 实现：`cpp/Common/include/Luau/VecDeque.h`。
//!
//! 移植说明：
//! - `queue[j]`（operator[]）与 `queue.at(j)` 共用 Rust `at`（带界检查），
//!   两条断言合并保留。
//! - 拷贝构造 → `clone()`（保留 head，可能不 contiguous）；拷贝赋值 →
//!   `clone_from()`（head 归零，总是 contiguous），与 C++ `operator=` 对应；
//!   移动构造/移动赋值 → Rust 所有权转移。
//! - `initializer_list` 构造 → `VecDeque::from_init_list`。
//! - `emplace_back/front` 的"原地构造、不调用拷贝构造"由 Rust 移动语义
//!   天然保证：`EmplaceOnly` 不实现 `Clone`，编译期即禁止隐式拷贝，
//!   `CHECK_NOTHROW` 退化为直接调用。
//! - SSO/非 SSO 字符串组保留原文本（短串栈内、长串堆上）。

/// C++ `testStringSet[2][10]`：第一组短串（打 SSO 路径），第二组长串。
const TEST_STRING_SET: [[&str; 10]; 2] = [
  [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
  ],
  [
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
  ],
];

mod forward_queue_test_no_initial_capacity {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:9:forward_queue_test_no_initial_capacity`
  //! Source: `tests/VecDeque.test.cpp:9-33`

  #[test]
  fn forward_queue_test_no_initial_capacity() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity is not set, so this should grow to be 11
    let mut queue: VecDeque<i32> = VecDeque::new();

    assert!(queue.empty());

    for i in 0..10 {
      queue.push_back(i);
    }
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    assert_eq!(queue.capacity(), 11);

    for j in 0..10i32 {
      assert_eq!(*queue.front(), j);
      assert_eq!(*queue.back(), 9);

      assert!(!queue.empty());
      queue.pop_front();
    }
  }
}

mod forward_queue_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:35:forward_queue_test`
  //! Source: `tests/VecDeque.test.cpp:35-60`

  #[test]
  fn forward_queue_test() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity set to 5 so that a grow is necessary
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.reserve(5);

    assert!(queue.empty());

    for i in 0..10 {
      queue.push_back(i);
    }
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    assert_eq!(queue.capacity(), 13);

    for j in 0..10i32 {
      assert_eq!(*queue.front(), j);
      assert_eq!(*queue.back(), 9);

      assert!(!queue.empty());
      queue.pop_front();
    }
  }
}

mod forward_queue_test_initializer_list {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:62:forward_queue_test_initializer_list`
  //! Source: `tests/VecDeque.test.cpp:62-80`

  #[test]
  fn forward_queue_test_initializer_list() {
    use ulua_common::records::vec_deque::VecDeque;

    let mut queue: VecDeque<i32> = VecDeque::from_init_list(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    assert_eq!(queue.capacity(), 10);

    for j in 0..10i32 {
      assert_eq!(*queue.front(), j);
      assert_eq!(*queue.back(), 9);

      assert!(!queue.empty());
      queue.pop_front();
    }
  }
}

mod reverse_queue_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:82:reverse_queue_test`
  //! Source: `tests/VecDeque.test.cpp:82-107`

  #[test]
  fn reverse_queue_test() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity set to 5 so that a grow is necessary
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.reserve(5);

    assert!(queue.empty());

    for i in 0..10 {
      queue.push_front(i);
    }
    // q: 9, 8, 7, 6, 5, 4, 3, 2, 1, 0

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    assert_eq!(queue.capacity(), 13);

    for j in 0..10i32 {
      assert_eq!(*queue.front(), 9);
      assert_eq!(*queue.back(), j);

      assert!(!queue.empty());
      queue.pop_back();
    }
  }
}

mod random_access_queue_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:109:random_access_queue_test`
  //! Source: `tests/VecDeque.test.cpp:109-129`
  //!
  //! C++ 的 `at(j)` 与 `queue[j]` 在 Rust 中同走 `at`（带界检查）。

  #[test]
  fn random_access_queue_test() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity set to 5 so that a grow is necessary
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.reserve(5);

    assert!(queue.empty());

    for i in 0..10 {
      queue.push_back(i);
    }
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    for j in 0..10usize {
      assert_eq!(*queue.at(j), j as i32);
      assert_eq!(*queue.at(j), j as i32);
    }
  }
}

mod clear_works_on_queue {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:131:clear_works_on_queue`
  //! Source: `tests/VecDeque.test.cpp:131-152`

  #[test]
  fn clear_works_on_queue() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity set to 5 so that a grow is necessary
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.reserve(5);

    assert!(queue.empty());

    for i in 0..10 {
      queue.push_back(i);
    }
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    for j in 0..10usize {
      assert_eq!(*queue.at(j), j as i32);
    }

    queue.clear();
    assert!(queue.empty());
    assert_eq!(queue.size(), 0);
  }
}

mod pop_front_at_end {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:154:pop_front_at_end`
  //! Source: `tests/VecDeque.test.cpp:154-180`

  #[test]
  fn pop_front_at_end() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity set to 5 so that a grow is necessary
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.reserve(5);

    assert!(queue.empty());

    // setting up the internal buffer to be: 1234567890 by the end (i.e. 0 at the end of the buffer)
    queue.push_front(0);

    for i in 1..10 {
      queue.push_back(i);
    }
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    for j in 0..10i32 {
      assert_eq!(*queue.front(), j);
      assert_eq!(*queue.back(), 9);

      assert!(!queue.empty());
      queue.pop_front();
    }
  }
}

mod pop_back_at_front {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:182:pop_back_at_front`
  //! Source: `tests/VecDeque.test.cpp:182-208`

  #[test]
  fn pop_back_at_front() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity set to 5 so that a grow is necessary
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.reserve(5);

    assert!(queue.empty());

    // setting up the internal buffer to be: 9012345678 by the end (i.e. 9 at the front of the buffer)
    queue.push_back(0);

    for i in 1..10 {
      queue.push_front(i);
    }
    // q: 9, 8, 7, 6, 5, 4, 3, 2, 1, 0

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    for j in 0..10i32 {
      assert_eq!(*queue.front(), 9);
      assert_eq!(*queue.back(), j);

      assert!(!queue.empty());
      queue.pop_back();
    }
  }
}

mod queue_is_contiguous {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:210:queue_is_contiguous`
  //! Source: `tests/VecDeque.test.cpp:210-226`

  #[test]
  fn queue_is_contiguous() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity is not set, so this should grow to be 11
    let mut queue: VecDeque<i32> = VecDeque::new();

    assert!(queue.empty());

    for i in 0..10 {
      queue.push_back(i);
    }
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    assert_eq!(queue.capacity(), 11);
    assert!(queue.is_contiguous());
  }
}

mod queue_is_not_contiguous {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:228:queue_is_not_contiguous`
  //! Source: `tests/VecDeque.test.cpp:228-251`

  #[test]
  fn queue_is_not_contiguous() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity is not set, so this should grow to be 11
    let mut queue: VecDeque<i32> = VecDeque::new();

    assert!(queue.empty());

    for i in 5..10 {
      queue.push_back(i);
    }
    for i in (0..5).rev() {
      queue.push_front(i);
    }
    // buffer: 56789......01234
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    assert_eq!(queue.capacity(), 11);
    assert!(!queue.is_contiguous());

    // checking that it is indeed sequential integers from 0 to 9
    for j in 0..10usize {
      assert_eq!(*queue.at(j), j as i32);
    }
  }
}

mod shrink_to_fit_works {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:253:shrink_to_fit_works`
  //! Source: `tests/VecDeque.test.cpp:253-288`

  #[test]
  fn shrink_to_fit_works() {
    use ulua_common::records::vec_deque::VecDeque;

    // initial capacity is not set, so this should grow to be 11
    let mut queue: VecDeque<i32> = VecDeque::new();

    assert!(queue.empty());

    for i in 5..10 {
      queue.push_back(i);
    }
    for i in (0..5).rev() {
      queue.push_front(i);
    }
    // buffer: 56789......01234
    // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

    assert!(!queue.empty());
    assert_eq!(queue.size(), 10);

    assert_eq!(queue.capacity(), 11);
    assert!(!queue.is_contiguous());

    // checking that it is indeed sequential integers from 0 to 9
    for j in 0..10usize {
      assert_eq!(*queue.at(j), j as i32);
    }

    queue.shrink_to_fit();
    // shrink to fit always makes a contiguous buffer
    assert!(queue.is_contiguous());
    // the capacity should be exactly the size now
    assert_eq!(queue.capacity(), queue.size());

    assert!(!queue.empty());

    // checking that it is still sequential integers from 0 to 9
    for j in 0..10usize {
      assert_eq!(*queue.at(j), j as i32);
    }
  }
}

mod string_queue_test_no_initial_capacity {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:308:string_queue_test_no_initial_capacity`
  //! Source: `tests/VecDeque.test.cpp:308-337`

  #[test]
  fn string_queue_test_no_initial_capacity() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity is not set, so this should grow to be 11
      let mut queue: VecDeque<String> = VecDeque::new();

      assert!(queue.empty());

      for ts in test_strings {
        queue.push_back(ts.to_string());
      }
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      assert_eq!(queue.capacity(), 11);

      for j in 0..10 {
        assert_eq!(queue.front().as_str(), test_strings[j]);
        assert_eq!(queue.back().as_str(), test_strings[9]);

        assert!(!queue.empty());
        queue.pop_front();
      }
    }
  }
}

mod string_queue_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:339:string_queue_test`
  //! Source: `tests/VecDeque.test.cpp:339-369`

  #[test]
  fn string_queue_test() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity set to 5 so that a grow is necessary
      let mut queue: VecDeque<String> = VecDeque::new();
      queue.reserve(5);

      assert!(queue.empty());

      for ts in test_strings {
        queue.push_back(ts.to_string());
      }
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      assert_eq!(queue.capacity(), 13);

      for j in 0..10 {
        assert_eq!(queue.front().as_str(), test_strings[j]);
        assert_eq!(queue.back().as_str(), test_strings[9]);

        assert!(!queue.empty());
        queue.pop_front();
      }
    }
  }
}

mod string_queue_test_initializer_list {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:371:string_queue_test_initializer_list`
  //! Source: `tests/VecDeque.test.cpp:371-405`

  #[test]
  fn string_queue_test_initializer_list() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      let mut queue: VecDeque<String> =
        VecDeque::from_init_list(test_strings.iter().map(|ts| ts.to_string()).collect());
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      assert_eq!(queue.capacity(), 10);

      for j in 0..10 {
        assert_eq!(queue.front().as_str(), test_strings[j]);
        assert_eq!(queue.back().as_str(), test_strings[9]);

        assert!(!queue.empty());
        queue.pop_front();
      }
    }
  }
}

mod reverse_string_queue_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:407:reverse_string_queue_test`
  //! Source: `tests/VecDeque.test.cpp:407-437`

  #[test]
  fn reverse_string_queue_test() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity set to 5 so that a grow is necessary
      let mut queue: VecDeque<String> = VecDeque::new();
      queue.reserve(5);

      assert!(queue.empty());

      for ts in test_strings {
        queue.push_front(ts.to_string());
      }
      // q: 9, 8, 7, 6, 5, 4, 3, 2, 1, 0

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      assert_eq!(queue.capacity(), 13);

      for j in 0..10 {
        assert_eq!(queue.front().as_str(), test_strings[9]);
        assert_eq!(queue.back().as_str(), test_strings[j]);

        assert!(!queue.empty());
        queue.pop_back();
      }
    }
  }
}

mod random_access_string_queue_test {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:439:random_access_string_queue_test`
  //! Source: `tests/VecDeque.test.cpp:439-464`

  #[test]
  fn random_access_string_queue_test() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity set to 5 so that a grow is necessary
      let mut queue: VecDeque<String> = VecDeque::new();
      queue.reserve(5);

      assert!(queue.empty());

      for ts in test_strings {
        queue.push_back(ts.to_string());
      }
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue.at(j).as_str(), *expected);
      }
    }
  }
}

mod clear_works_on_string_queue {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:466:clear_works_on_string_queue`
  //! Source: `tests/VecDeque.test.cpp:466-492`

  #[test]
  fn clear_works_on_string_queue() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity set to 5 so that a grow is necessary
      let mut queue: VecDeque<String> = VecDeque::new();
      queue.reserve(5);

      assert!(queue.empty());

      for ts in test_strings {
        queue.push_back(ts.to_string());
      }
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue.at(j).as_str(), *expected);
      }

      queue.clear();
      assert!(queue.empty());
      assert_eq!(queue.size(), 0);
    }
  }
}

mod pop_front_string_at_end {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:494:pop_front_string_at_end`
  //! Source: `tests/VecDeque.test.cpp:494-525`

  #[test]
  fn pop_front_string_at_end() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity set to 5 so that a grow is necessary
      let mut queue: VecDeque<String> = VecDeque::new();
      queue.reserve(5);

      assert!(queue.empty());

      // setting up the internal buffer to be: 1234567890 by the end (i.e. 0 at the end of the buffer)
      queue.push_front(test_strings[0].to_string());

      for ts in &test_strings[1..] {
        queue.push_back(ts.to_string());
      }
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      for j in 0..10 {
        assert_eq!(queue.front().as_str(), test_strings[j]);
        assert_eq!(queue.back().as_str(), test_strings[9]);

        assert!(!queue.empty());
        queue.pop_front();
      }
    }
  }
}

mod pop_back_string_at_front {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:527:pop_back_string_at_front`
  //! Source: `tests/VecDeque.test.cpp:527-558`

  #[test]
  fn pop_back_string_at_front() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity set to 5 so that a grow is necessary
      let mut queue: VecDeque<String> = VecDeque::new();
      queue.reserve(5);

      assert!(queue.empty());

      // setting up the internal buffer to be: 9012345678 by the end (i.e. 9 at the front of the buffer)
      queue.push_back(test_strings[0].to_string());

      for ts in &test_strings[1..] {
        queue.push_front(ts.to_string());
      }
      // q: 9, 8, 7, 6, 5, 4, 3, 2, 1, 0

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      for j in 0..10 {
        assert_eq!(queue.front().as_str(), test_strings[9]);
        assert_eq!(queue.back().as_str(), test_strings[j]);

        assert!(!queue.empty());
        queue.pop_back();
      }
    }
  }
}

mod string_queue_is_contiguous {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:560:string_queue_is_contiguous`
  //! Source: `tests/VecDeque.test.cpp:560-634`
  //!
  //! 拷贝构造 → `clone`；拷贝赋值 → `clone_from`；移动构造/移动赋值 →
  //! 所有权转移。

  #[test]
  fn string_queue_is_contiguous() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity is not set, so this should grow to be 11
      let mut queue: VecDeque<String> = VecDeque::new();

      assert!(queue.empty());

      for ts in test_strings {
        queue.push_back(ts.to_string());
      }
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      assert_eq!(queue.capacity(), 11);
      assert!(queue.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue.at(j).as_str(), *expected);
      }

      // Check copy construction
      let queue2 = queue.clone();

      assert!(!queue2.empty());
      assert_eq!(queue2.size(), 10);

      assert_eq!(queue2.capacity(), 11);
      assert!(queue2.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue2.at(j).as_str(), *expected);
      }

      // Check copy assignment
      let mut queue3: VecDeque<String> = VecDeque::new();
      queue3.clone_from(&queue);

      assert!(!queue3.empty());
      assert_eq!(queue3.size(), 10);

      assert_eq!(queue3.capacity(), 11);
      assert!(queue3.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue3.at(j).as_str(), *expected);
      }

      // Check move construction
      let queue4 = queue3;

      assert!(!queue4.empty());
      assert_eq!(queue4.size(), 10);

      assert_eq!(queue4.capacity(), 11);
      assert!(queue4.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue4.at(j).as_str(), *expected);
      }

      // Check move assignment
      let queue5 = queue2;

      assert!(!queue5.empty());
      assert_eq!(queue5.size(), 10);

      assert_eq!(queue5.capacity(), 11);
      assert!(queue5.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue5.at(j).as_str(), *expected);
      }
    }
  }
}

mod string_queue_is_not_contiguous {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:636:string_queue_is_not_contiguous`
  //! Source: `tests/VecDeque.test.cpp:636-729`
  //!
  //! 本用例区分拷贝构造（保留 head，不 contiguous）与拷贝赋值
  //! （head 归零，contiguous）——分别对应 `clone` 与 `clone_from`。

  #[test]
  fn string_queue_is_not_contiguous() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity is not set, so this should grow to be 11
      let mut queue: VecDeque<String> = VecDeque::new();

      assert!(queue.empty());

      for ts in &test_strings[5..] {
        queue.push_back(ts.to_string());
      }
      for ts in test_strings[..5].iter().rev() {
        queue.push_front(ts.to_string());
      }
      // buffer: 56789......01234
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      assert_eq!(queue.capacity(), 11);
      assert!(!queue.is_contiguous());

      // checking that it is indeed sequential integers from 0 to 9
      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue.at(j).as_str(), *expected);
      }

      // Check copy construction
      let queue2 = queue.clone();

      assert!(!queue2.empty());
      assert_eq!(queue2.size(), 10);

      assert_eq!(queue2.capacity(), 11);
      assert!(!queue2.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue2.at(j).as_str(), *expected);
      }

      // Check copy assignment
      let mut queue3: VecDeque<String> = VecDeque::new();
      queue3.clone_from(&queue);

      assert!(!queue3.empty());
      assert_eq!(queue3.size(), 10);

      assert_eq!(queue3.capacity(), 11);
      assert!(queue3.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue3.at(j).as_str(), *expected);
      }

      // Check move construction
      let queue4 = queue;

      assert!(!queue4.empty());
      assert_eq!(queue4.size(), 10);

      assert_eq!(queue4.capacity(), 11);
      assert!(!queue4.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue4.at(j).as_str(), *expected);
      }

      // Check move assignment
      let queue5: VecDeque<String> = queue2;

      assert!(!queue5.empty());
      assert_eq!(queue5.size(), 10);

      assert_eq!(queue5.capacity(), 11);
      assert!(!queue5.is_contiguous());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue5.at(j).as_str(), *expected);
      }

      // Check that grow from discontiguous is handled well
      let mut queue4 = queue4;
      queue4.push_back("zero".to_string());
      queue4.push_back("?".to_string());

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue4.at(j).as_str(), *expected);
      }
      assert_eq!(queue4.at(10).as_str(), "zero");
      assert_eq!(queue4.at(11).as_str(), "?");

      // Check that reserve from discontiguous is handled well
      let mut queue5 = queue5;
      queue5.reserve(20);

      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue5.at(j).as_str(), *expected);
      }
    }
  }
}

mod shrink_to_fit_works_with_strings {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:731:shrink_to_fit_works_with_strings`
  //! Source: `tests/VecDeque.test.cpp:731-771`

  #[test]
  fn shrink_to_fit_works_with_strings() {
    use ulua_common::records::vec_deque::VecDeque;

    use crate::TEST_STRING_SET;

    for test_strings in TEST_STRING_SET {
      // initial capacity is not set, so this should grow to be 11
      let mut queue: VecDeque<String> = VecDeque::new();

      assert!(queue.empty());

      for ts in &test_strings[5..] {
        queue.push_back(ts.to_string());
      }
      for ts in test_strings[..5].iter().rev() {
        queue.push_front(ts.to_string());
      }
      // buffer: 56789......01234
      // q: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9

      assert!(!queue.empty());
      assert_eq!(queue.size(), 10);

      assert_eq!(queue.capacity(), 11);
      assert!(!queue.is_contiguous());

      // checking that it is indeed sequential integers from 0 to 9
      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue.at(j).as_str(), *expected);
      }

      queue.shrink_to_fit();
      // shrink to fit always makes a contiguous buffer
      assert!(queue.is_contiguous());
      // the capacity should be exactly the size now
      assert_eq!(queue.capacity(), queue.size());

      assert!(!queue.empty());

      // checking that it is still sequential integers from 0 to 9
      for (j, expected) in test_strings.iter().enumerate() {
        assert_eq!(queue.at(j).as_str(), *expected);
      }
    }
  }
}

mod push_front_elements_are_destroyed_correctly {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:778:push_front_elements_are_destroyed_correctly`
  //! Source: `tests/VecDeque.test.cpp:778-798`
  //!
  //! `shared_ptr` 引用计数镜像为 `Arc::strong_count`。

  struct TestStruct;

  #[test]
  fn push_front_elements_are_destroyed_correctly() {
    use std::sync::Arc;

    use ulua_common::records::vec_deque::VecDeque;

    // Verify that elements pushed to the front of the queue are properly destroyed when the queue is destroyed.
    let t = Arc::new(TestStruct);
    {
      let mut queue: VecDeque<Arc<TestStruct>> = VecDeque::new();
      assert!(queue.empty());
      queue.reserve(10);
      queue.push_front(t.clone());
      queue.push_front(t.clone());
      assert_eq!(Arc::strong_count(&t), 3); // Num of references to the TestStruct instance is now 3
      // <-- call destructor here

      // Extra check for correct copies
      let _queue2 = queue.clone();
      let mut queue3: VecDeque<Arc<TestStruct>> = VecDeque::new();
      queue3.clone_from(&queue);
    }

    // At this point the destructor should be called and we should be back down to one instance of TestStruct
    assert_eq!(Arc::strong_count(&t), 1);
  }
}

mod emplace_back_constructs_in_place {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:837:emplace_back_constructs_in_place`
  //! Source: `tests/VecDeque.test.cpp:837-849`
  //!
  //! C++ 用"拷贝构造即 throw"的 `EmplaceOnly` 验证原地构造；Rust 移动语义
  //! 下 `EmplaceOnly` 不实现 `Clone` 即编译期禁止拷贝，`push_back` 即
  //! `emplace_back`。

  struct EmplaceOnly {
    x: i32,
    y: i32,
  }

  #[test]
  fn emplace_back_constructs_in_place() {
    use ulua_common::records::vec_deque::VecDeque;

    let mut queue: VecDeque<EmplaceOnly> = VecDeque::new();

    // CHECK_NOTHROW(queue.emplace_back(10, 20));
    queue.push_back(EmplaceOnly { x: 10, y: 20 });
    // CHECK_NOTHROW(queue.emplace_back(30, 40));
    queue.push_back(EmplaceOnly { x: 30, y: 40 });

    assert_eq!(queue.size(), 2);
    assert_eq!(queue.at(0).x, 10);
    assert_eq!(queue.at(0).y, 20);
    assert_eq!(queue.at(1).x, 30);
    assert_eq!(queue.at(1).y, 40);
  }
}

mod emplace_front_constructs_in_place {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:851:emplace_front_constructs_in_place`
  //! Source: `tests/VecDeque.test.cpp:851-863`

  struct EmplaceOnly {
    x: i32,
    y: i32,
  }

  #[test]
  fn emplace_front_constructs_in_place() {
    use ulua_common::records::vec_deque::VecDeque;

    let mut queue: VecDeque<EmplaceOnly> = VecDeque::new();

    // CHECK_NOTHROW(queue.emplace_front(10, 20));
    queue.push_front(EmplaceOnly { x: 10, y: 20 });
    // CHECK_NOTHROW(queue.emplace_front(30, 40));
    queue.push_front(EmplaceOnly { x: 30, y: 40 });

    assert_eq!(queue.size(), 2);
    assert_eq!(queue.at(0).x, 30);
    assert_eq!(queue.at(0).y, 40);
    assert_eq!(queue.at(1).x, 10);
    assert_eq!(queue.at(1).y, 20);
  }
}

mod emplace_mixed_front_and_back {
  //! Node: `cxx:Test:Luau.UnitTest:tests/VecDeque.test.cpp:865:emplace_mixed_front_and_back`
  //! Source: `tests/VecDeque.test.cpp:865-887`

  struct EmplaceOnly {
    x: i32,
    y: i32,
  }

  #[test]
  fn emplace_mixed_front_and_back() {
    use ulua_common::records::vec_deque::VecDeque;

    let mut queue: VecDeque<EmplaceOnly> = VecDeque::new();

    queue.push_back(EmplaceOnly { x: 1, y: 2 });
    queue.push_front(EmplaceOnly { x: 3, y: 4 });
    queue.push_back(EmplaceOnly { x: 5, y: 6 });
    queue.push_back(EmplaceOnly { x: 7, y: 8 });
    queue.push_front(EmplaceOnly { x: 9, y: 10 });
    // expected order: (9,10), (3,4), (1,2), (5,6), (7,8)

    assert_eq!(queue.size(), 5);
    assert_eq!(queue.at(0).x, 9);
    assert_eq!(queue.at(0).y, 10);
    assert_eq!(queue.at(1).x, 3);
    assert_eq!(queue.at(1).y, 4);
    assert_eq!(queue.at(2).x, 1);
    assert_eq!(queue.at(2).y, 2);
    assert_eq!(queue.at(3).x, 5);
    assert_eq!(queue.at(3).y, 6);
    assert_eq!(queue.at(4).x, 7);
    assert_eq!(queue.at(4).y, 8);
  }
}

// ---- 补充用例（自 ulua-common/tests/vec_deque.rs 合并；cpp 无对应 TEST_CASE，
// 覆盖 iter / iter_mut / into_iter 的逻辑序与环绕行为）----

mod iter_yields_logical_order {
  #[test]
  fn iter_yields_logical_order() {
    use ulua_common::records::vec_deque::VecDeque;

    // 无环绕：单段连续。
    let queue = VecDeque::from_init_list(vec![0, 1, 2, 3, 4]);
    let collected: Vec<i32> = queue.iter().copied().collect();
    assert_eq!(collected, vec![0, 1, 2, 3, 4]);
    assert_eq!(queue.iter().len(), 5);
    assert_eq!(queue.iter().count(), 5);
  }
}

mod iter_yields_logical_order_wrapped {
  #[test]
  fn iter_yields_logical_order_wrapped() {
    use ulua_common::records::vec_deque::VecDeque;

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
}

mod iter_mut_modifies_elements_in_place {
  #[test]
  fn iter_mut_modifies_elements_in_place() {
    use ulua_common::records::vec_deque::VecDeque;

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
}

mod into_iter_consumes_in_logical_order_and_drops_rest {
  #[test]
  fn into_iter_consumes_in_logical_order_and_drops_rest() {
    use std::rc::Rc;

    use ulua_common::records::vec_deque::VecDeque;

    struct TestStruct;

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
}

mod into_iterator_impls_match_std_semantics {
  #[test]
  fn into_iterator_impls_match_std_semantics() {
    use ulua_common::records::vec_deque::VecDeque;

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
}
