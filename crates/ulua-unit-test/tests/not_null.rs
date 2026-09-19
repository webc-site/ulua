extern crate alloc;

mod not_null_basic_stuff {
  //! Source: `tests/NotNull.test.cpp`

  #[test]
  fn not_null_basic_stuff() {
    use core::f32::consts::PI;

    use ulua_analysis::records::not_null::NotNull;
    use ulua_unit_test::records::test::Test;

    fn bar(_q: *mut i32) {}

    let mut a_box = Box::new(55);
    let mut b_box = Box::new(55);

    let a = NotNull::new(&mut *a_box as *mut i32);
    let b = NotNull::new(&mut *b_box as *mut i32);

    let mut d = a;

    let e = *d;
    *d = 1;
    assert_eq!(e, 55);

    let f = d;
    unsafe {
      *f.get() = 5;
    }

    assert_eq!(a, d);
    assert_ne!(a, b);

    let g = a;
    assert_eq!(g, a);

    let mut t_box = Box::new(Test::new());
    let t = NotNull::new(&mut *t_box as *mut Test);
    unsafe {
      (*t.get()).x = 5;
      (*t.get()).y = PI;
    }

    let u = t;
    unsafe {
      (*u.get()).x = 44;
    }
    let v = unsafe { (*u.get()).x };
    assert_eq!(v, 44);

    bar(a.get());

    drop(a_box);
    drop(b_box);
    drop(t_box);

    assert_eq!(0, Test::count());
  }
}

mod not_null_const {
  //! Source: `tests/NotNull.test.cpp`

  #[test]
  fn not_null_const() {
    use ulua_analysis::records::not_null::NotNull;

    let mut p = 0;
    let mut q = 0;

    let n = NotNull::new(&mut p as *mut i32);

    unsafe {
      *n.get() = 123;
    }

    let mut m = n;

    assert_eq!(123, *m);

    let n2 = NotNull::new(&mut q as *mut i32);
    m = n2;

    let m2 = n;
    unsafe {
      *m2.get() = 321;
    }

    assert_eq!(321, *n);
    assert_eq!(m.get(), n2.get());
  }
}

mod not_null_const_compatibility {
  //! Source: `tests/NotNull.test.cpp`

  #[test]
  fn not_null_const_compatibility() {
    use ulua_analysis::records::not_null::NotNull;

    let mut raw = Box::new(8);

    let a = NotNull::new(&mut *raw as *mut i32);
    let _b = NotNull::new(&mut *raw as *mut i32);
    let c = a;

    assert_eq!(*c, 8);
  }
}

mod not_null_hashable {
  //! Source: `tests/NotNull.test.cpp`

  #[test]
  fn not_null_hashable() {
    use std::collections::HashMap;

    use ulua_analysis::records::not_null::NotNull;

    let mut a_ = 8;
    let mut b_ = 10;

    let a = NotNull::new(&mut a_ as *mut i32);
    let b = NotNull::new(&mut b_ as *mut i32);

    let hello = "hello";
    let world = "world";

    let mut map = HashMap::new();
    map.insert(a, hello);
    map.insert(b, world);

    assert_eq!(2, map.len());
    assert_eq!(hello, map[&a]);
    assert_eq!(world, map[&b]);
  }
}
