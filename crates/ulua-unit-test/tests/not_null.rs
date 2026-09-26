extern crate alloc;

// Source: `tests/NotNull.test.cpp`
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

  let d = a;

  let e = *d;
  // cpp's `*d = 1;` — NotNull is Copy, so mutation goes through get()
  // rather than DerefMut (see records/not_null.rs deviation note).
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`b_box` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    *d.get() = 1;
  }
  assert_eq!(e, 55);

  let f = d;
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`b_box` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    *f.get() = 5;
  }

  assert_eq!(a, d);
  assert_ne!(a, b);

  let g = a;
  assert_eq!(g, a);

  let mut t_box = Box::new(Test::new());
  let t = NotNull::new(&mut *t_box as *mut Test);
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`t_box` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    (*t.get()).x = 5;
    (*t.get()).y = PI;
  }

  let u = t;
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`t_box` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    (*u.get()).x = 44;
  }
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`t_box` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
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

  let n = NotNull::new(&mut p as *mut i32);

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`p` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    *n.get() = 123;
  }

  let mut m = n;

  assert_eq!(123, *m);

  let n2 = NotNull::new(&mut q as *mut i32);
  m = n2;

  let m2 = n;
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`q` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
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

  let a = NotNull::new(&mut *raw as *mut i32);
  let _b = NotNull::new(&mut *raw as *mut i32);
  let c = a;

  assert_eq!(*c, 8);
}

// Source: `tests/NotNull.test.cpp`
#[test]
fn not_null_hashable() {
  use ulua_analysis::records::not_null::NotNull;
  use ulua_common::collections::HashMap;

  let mut a_ = 8;
  let mut b_ = 10;

  let a = NotNull::new(&mut a_ as *mut i32);
  let b = NotNull::new(&mut b_ as *mut i32);

  let hello = "hello";
  let world = "world";

  let mut map: HashMap<_, _> = HashMap::default();
  map.insert(a, hello);
  map.insert(b, world);

  assert_eq!(2, map.len());
  assert_eq!(hello, map[&a]);
  assert_eq!(world, map[&b]);
}
