//! Faithful runtime shape for `Luau::NotNull<T>` (`Analysis/include/Luau/NotNull.h`).
//!
//! Deviation from cpp: `NotNull` deliberately does NOT implement `DerefMut`.
//! cpp's `operator*` returns `T&`, but in Rust this type is `Copy`, so
//! `&mut NotNull<T>` could be copied to yield aliasing `&mut` to the pointee
//! (UB). Mutation goes through `get()` with an explicit unsafe borrow.

use core::{
  hash::{Hash, Hasher},
  ops::Deref,
  ptr::NonNull,
};
#[repr(transparent)]
#[derive(Debug)]
pub struct NotNull<T: ?Sized> {
  ptr: NonNull<T>,
}

impl<T: ?Sized> Copy for NotNull<T> {}

impl<T: ?Sized> Clone for NotNull<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T: ?Sized> NotNull<T> {
  pub fn new(ptr: *mut T) -> Self {
    Self {
      ptr: NonNull::new(ptr).expect("NotNull constructed from null pointer"),
    }
  }

  pub fn from_ref(value: &mut T) -> Self {
    Self::new(value as *mut T)
  }

  pub fn get(self) -> *mut T {
    self.ptr.as_ptr()
  }
}

impl<T: ?Sized> Deref for NotNull<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    // Safety: `self.ptr` 是 `NonNull<T>`，由 `NotNull::new`（对 null 走 `expect` panic）保证
    // 非空且对齐；`from_ref` 更是直接从活的 `&mut T` 取得。`NotNull` 的不变量是其目标在自身
    // 存活期内一直有效，故重建共享引用 `&T`（生命周期即 `&self`）合法。
    unsafe { self.ptr.as_ref() }
  }
}

impl<T: ?Sized, U: ?Sized> PartialEq<NotNull<U>> for NotNull<T> {
  fn eq(&self, other: &NotNull<U>) -> bool {
    self.ptr.as_ptr().cast::<()>() == other.ptr.as_ptr().cast::<()>()
  }
}

impl<T: ?Sized> Eq for NotNull<T> {}

impl<T: ?Sized> Hash for NotNull<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.ptr.as_ptr().cast::<()>().hash(state);
  }
}
