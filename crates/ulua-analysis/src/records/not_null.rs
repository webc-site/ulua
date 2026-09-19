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
