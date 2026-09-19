extern crate alloc;

mod variant_create {
  //! Source: `tests/Variant.test.cpp`

  #[test]
  fn variant_create() {
    use ulua_common::records::variant::Variant2;
    use ulua_unit_test::records::foo::Foo;

    let v1: Variant2<i32, Foo> = Variant2::V0(1);
    let v2: Variant2<Foo, i32> = Variant2::V0(Foo { x: 2 });

    let f = Foo { x: 3 };
    let v3: Variant2<Foo, i32> = Variant2::V0(f);

    assert!(v1.get_if_0().is_some());
    assert_eq!(*v1.get_if_0().unwrap(), 1);

    assert!(v2.get_if_0().is_some());
    assert_eq!(v2.get_if_0().unwrap().x, 2);

    assert!(v3.get_if_0().is_some());
    assert_eq!(v3.get_if_0().unwrap().x, 3);
  }
}

mod variant_default_ctor {
  //! Source: `tests/Variant.test.cpp`

  #[test]
  fn variant_default_ctor() {
    use ulua_common::records::variant::Variant2;
    use ulua_unit_test::records::foo::Foo;

    let v1: Variant2<i32, Foo> = Variant2::default();
    let v2: Variant2<Foo, i32> = Variant2::default();

    assert!(v1.get_if_0().is_some());
    assert_eq!(*v1.get_if_0().unwrap(), 0);
    assert!(v1.get_if_1().is_none());

    assert!(v2.get_if_0().is_some());
    assert_eq!(v2.get_if_0().unwrap().x, 42);
  }
}

mod variant_equality {
  //! Source: `tests/Variant.test.cpp`

  #[test]
  fn variant_equality() {
    use alloc::string::String;

    use ulua_common::records::variant::Variant2;

    let v1: Variant2<i32, String> = Variant2::V1(String::from("hi"));
    let v2: Variant2<i32, String> = Variant2::V1(String::from("me"));
    let v3: Variant2<i32, String> = Variant2::V0(1);
    let v4: Variant2<i32, String> = Variant2::V0(0);
    let v5: Variant2<i32, String> = Variant2::default();

    assert_eq!(v1, v1);
    assert_ne!(v1, v2);
    assert_ne!(v1, v3);
    assert_ne!(v3, v4);
    assert_eq!(v4, v5);
  }
}

mod variant_move {
  //! Source: `tests/Variant.test.cpp`

  #[test]
  fn variant_move() {
    use ulua_common::records::variant::Variant1;
    use ulua_unit_test::records::move_only::MoveOnly;

    let v1: Variant1<MoveOnly> = Variant1::V0(MoveOnly);
    let _v2 = v1;
  }
}

mod variant_move_with_copyable_alternative {
  //! Source: `tests/Variant.test.cpp`

  #[test]
  fn variant_move_with_copyable_alternative() {
    use alloc::string::String;
    use core::mem::take;

    use ulua_common::records::variant::Variant2;
    use ulua_unit_test::records::move_only::MoveOnly;

    let mut v1: Variant2<String, MoveOnly> = Variant2::V0(String::from(
      "Hello, world! I am longer than a normal hello world string to avoid SSO.",
    ));
    let moved = take(v1.get_if_0_mut().unwrap());
    let v2: Variant2<String, MoveOnly> = Variant2::V0(moved);

    let s1 = v1.get_if_0();
    assert!(s1.is_some());
    assert_eq!(s1.unwrap(), "");

    let s2 = v2.get_if_0();
    assert!(s2.is_some());
    assert_eq!(
      s2.unwrap(),
      "Hello, world! I am longer than a normal hello world string to avoid SSO."
    );
  }
}

mod variant_emplace {
  //! Source: `tests/Variant.test.cpp`

  use core::sync::atomic::{AtomicUsize, Ordering};

  /// 对齐 cpp `Bar::count`：存活实例计数（构造 +1 / drop -1）
  static BAR_COUNT: AtomicUsize = AtomicUsize::new(0);

  struct Bar {
    prop: i32,
  }

  impl Bar {
    fn new(x: i32) -> Self {
      BAR_COUNT.fetch_add(1, Ordering::SeqCst);
      Bar { prop: x * 2 }
    }
  }

  impl Drop for Bar {
    fn drop(&mut self) {
      BAR_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
  }

  #[test]
  fn variant_emplace() {
    use ulua_common::records::variant::Variant2;

    {
      // emplace 即变体重赋值：旧值被 drop，新值就位
      let mut v1: Variant2<i32, Bar> = Variant2::V0(5);

      assert_eq!(0, BAR_COUNT.load(Ordering::SeqCst));
      assert_eq!(5, *v1.get_if_0().unwrap());
      assert_eq!(0, BAR_COUNT.load(Ordering::SeqCst));

      v1 = Variant2::V1(Bar::new(11));
      assert_eq!(22, v1.get_if_1().unwrap().prop);
      assert_eq!(1, BAR_COUNT.load(Ordering::SeqCst));
      assert!(v1.get_if_1().is_some());
      assert!(v1.get_if_0().is_none());
    }

    // 作用域结束：持有 Bar 的 Variant 已析构
    assert_eq!(0, BAR_COUNT.load(Ordering::SeqCst));
  }
}

mod variant_non_pod {
  //! Source: `tests/Variant.test.cpp`

  #[test]
  fn variant_non_pod() {
    use alloc::string::String;
    use core::mem::take;

    use ulua_common::records::variant::Variant2;

    let s1 = String::from("hello");
    let v1: Variant2<String, i32> = Variant2::V0(s1.clone());

    assert_eq!(v1.get_if_0().unwrap(), "hello");

    let mut v2: Variant2<String, i32> = Variant2::V0(String::from("hello"));

    assert_eq!(v2.get_if_0().unwrap(), "hello");

    v2 = Variant2::V0(String::from(
      "this is a long string that doesn't fit into the small buffer",
    ));

    assert_eq!(
      v2.get_if_0().unwrap(),
      "this is a long string that doesn't fit into the small buffer"
    );

    let s2 = String::from("this is another long string, and this time we're copying it");
    v2 = Variant2::V0(s2.clone());

    assert_eq!(
      v2.get_if_0().unwrap(),
      "this is another long string, and this time we're copying it"
    );

    let mut v3 = v2.clone();

    assert_eq!(
      v2.get_if_0().unwrap(),
      "this is another long string, and this time we're copying it"
    );
    assert_eq!(
      v3.get_if_0().unwrap(),
      "this is another long string, and this time we're copying it"
    );

    let moved = take(v3.get_if_0_mut().unwrap());
    let v4: Variant2<String, i32> = Variant2::V0(moved);

    assert_eq!(
      v2.get_if_0().unwrap(),
      "this is another long string, and this time we're copying it"
    );
    assert_eq!(v3.get_if_0().unwrap(), "");
    assert_eq!(
      v4.get_if_0().unwrap(),
      "this is another long string, and this time we're copying it"
    );
  }
}

mod variant_visit {
  //! Source: `tests/Variant.test.cpp`

  #[test]
  fn variant_visit() {
    use alloc::string::String;

    use ulua_common::records::variant::Variant2;
    use ulua_unit_test::records::{
      increment_visitor::IncrementVisitor, to_string_visitor::ToStringVisitor,
    };

    let mut v1: Variant2<String, i32> = Variant2::V0(String::from("123"));
    let mut v2: Variant2<String, i32> = Variant2::V1(45);
    let v1c = &v1;
    let v2c = &v2;

    let to_string_visitor = ToStringVisitor::default();
    let increment_visitor = IncrementVisitor::default();

    let mut r1 = String::new();
    match v1c {
      Variant2::V0(v) => r1.push_str(&to_string_visitor.operator_call(v)),
      Variant2::V1(v) => r1.push_str(&to_string_visitor.operator_call_mut(*v)),
    }
    match v2c {
      Variant2::V0(v) => r1.push_str(&to_string_visitor.operator_call(v)),
      Variant2::V1(v) => r1.push_str(&to_string_visitor.operator_call_mut(*v)),
    }
    assert_eq!(r1, "12345");

    let mut r2 = String::new();
    r2.push_str(&match v1c {
      Variant2::V0(v) => to_string_visitor.operator_call(v),
      Variant2::V1(v) => to_string_visitor.operator_call_mut(*v),
    });
    r2.push_str(&match v2c {
      Variant2::V0(v) => to_string_visitor.operator_call(v),
      Variant2::V1(v) => to_string_visitor.operator_call_mut(*v),
    });
    assert_eq!(r2, "12345");

    match &mut v1 {
      Variant2::V0(v) => increment_visitor.operator_call(v),
      Variant2::V1(v) => increment_visitor.operator_call_mut(v),
    }
    match &mut v2 {
      Variant2::V0(v) => increment_visitor.operator_call(v),
      Variant2::V1(v) => increment_visitor.operator_call_mut(v),
    }
    assert_eq!(
      match &v1 {
        Variant2::V0(v) => to_string_visitor.operator_call(v),
        Variant2::V1(v) => to_string_visitor.operator_call_mut(*v),
      },
      "1231"
    );
    assert_eq!(
      match &v2 {
        Variant2::V0(v) => to_string_visitor.operator_call(v),
        Variant2::V1(v) => to_string_visitor.operator_call_mut(*v),
      },
      "46"
    );

    let mut r3 = String::new();
    r3.push_str(&match &mut v1 {
      Variant2::V0(v) => {
        increment_visitor.operator_call(v);
        to_string_visitor.operator_call(v)
      }
      Variant2::V1(v) => {
        increment_visitor.operator_call_mut(v);
        to_string_visitor.operator_call_mut(*v)
      }
    });
    r3.push_str(&match &mut v2 {
      Variant2::V0(v) => {
        increment_visitor.operator_call(v);
        to_string_visitor.operator_call(v)
      }
      Variant2::V1(v) => {
        increment_visitor.operator_call_mut(v);
        to_string_visitor.operator_call_mut(*v)
      }
    });
    assert_eq!(r3, "1231147");
  }
}
