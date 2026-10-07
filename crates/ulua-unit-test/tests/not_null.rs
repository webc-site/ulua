//! `cpp/tests/NotNull.test.cpp` 的移植。
//!
//! 被测对象就是 `NotNull<T>` 本身：`get()` 交回的裸指针即该类型的公共契约面，故
//! 保留；但构造一律走 `NotNull::from_ref`（不再手写 `&mut x as *mut _` 的指针折算），
//! 每处 `unsafe` 只包住「经 `get()` 写目标」这一句真实前置条件所在的最窄范围。

extern crate alloc;

// Source: `tests/NotNull.test.cpp`
#[test]
fn not_null_basic_stuff() {
  use core::f32::consts::PI;

  use ulua_analysis::records::not_null::NotNull;
  use ulua_unit_test::records::test::Test;

  /// 被测点：`get()` 的返回值可直接喂给 C 形签名（cpp 侧原样调用 `bar(q)`）。
  fn bar(_q: *mut i32) {}

  let mut a_box = Box::new(55);
  let mut b_box = Box::new(55);

  let a = NotNull::from_ref(&mut *a_box);
  let b = NotNull::from_ref(&mut *b_box);

  let d = a;

  let e = *d;
  // cpp 的 `*d = 1;`。`NotNull` 是 `Copy`，故刻意不实现 `DerefMut`（否则复制
  // `&mut NotNull` 会得到两个别名 `&mut`，见 records/not_null.rs 的偏离说明），
  // 写操作必须经 `get()`。
  // Safety: `d` 由非空不变式构造，指向本用例独占、直到此处仍存活的 `a_box` 内容。
  unsafe {
    *d.get() = 1;
  }
  assert_eq!(e, 55);

  let f = d;
  // Safety: 同 `d`——`f` 是同一 `&mut a_box` 的副本，目标仍存活。
  unsafe {
    *f.get() = 5;
  }

  assert_eq!(a, d);
  assert_ne!(a, b);

  let g = a;
  assert_eq!(g, a);

  let mut t_box = Box::new(Test::new());
  let t = NotNull::from_ref(&mut *t_box);
  // Safety: `t` 指向本用例独占且存活的 `t_box` 内容，`get()` 写目标即写该 Box。
  unsafe {
    (*t.get()).x = 5;
    (*t.get()).y = PI;
  }

  let u = t;
  // Safety: 同上，`u` 为 `t` 的副本。
  unsafe {
    (*u.get()).x = 44;
  }
  // Safety: 同上，只读回同一存活目标。
  let v = unsafe { (*u.get()).x };
  assert_eq!(v, 44);

  bar(a.get());

  drop(a_box);
  drop(b_box);
  drop(t_box);

  assert_eq!(0, Test::count());
}

// Source: `tests/NotNull.test.cpp`
#[test]
fn not_null_const() {
  use ulua_analysis::records::not_null::NotNull;

  let mut p = 0;
  let mut q = 0;

  let n = NotNull::from_ref(&mut p);

  // Safety: `n` 指向本用例独占且存活的局部 `p`。
  unsafe {
    *n.get() = 123;
  }

  let mut m = n;

  assert_eq!(123, *m);

  let n2 = NotNull::from_ref(&mut q);
  m = n2;

  let m2 = n;
  // Safety: `m2` 仍是 `p` 的副本，`n`/`p` 未释放。
  unsafe {
    *m2.get() = 321;
  }

  assert_eq!(321, *n);
  assert_eq!(m.get(), n2.get());
}

// Source: `tests/NotNull.test.cpp`
#[test]
fn not_null_const_compatibility() {
  use ulua_analysis::records::not_null::NotNull;

  let mut raw = Box::new(8);

  let a = NotNull::from_ref(&mut *raw);
  let _b = NotNull::from_ref(&mut *raw);
  let c = a;

  assert_eq!(*c, 8);
}

// Source: `tests/NotNull.test.cpp`
#[test]
fn not_null_hashable() {
  use hashbrown::HashMap;
  use ulua_analysis::records::not_null::NotNull;

  let mut a_ = 8;
  let mut b_ = 10;

  let a = NotNull::from_ref(&mut a_);
  let b = NotNull::from_ref(&mut b_);

  let hello = "hello";
  let world = "world";

  let mut map: HashMap<_, _> = HashMap::default();
  map.insert(a, hello);
  map.insert(b, world);

  assert_eq!(2, map.len());
  assert_eq!(hello, map[&a]);
  assert_eq!(world, map[&b]);
}
