/// Port of `Luau::Unifiable::Error<Id>` from `Analysis/include/Luau/Unifiable.h`.
/// An error type with an optional "synthetic" stand-in TypeId for presentation.
use core::sync::atomic::AtomicI32;
use core::sync::atomic::Ordering;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error<Id> {
  pub index: i32,
  /// Optional synthetic TypeId used to communicate the error to the user.
  pub synthetic: Option<Id>,
}

impl<Id: Copy> Error<Id> {
  pub fn new() -> Self {
    static NEXT_INDEX: AtomicI32 = AtomicI32::new(0);
    Self {
      index: NEXT_INDEX.fetch_add(1, Ordering::Relaxed),
      synthetic: None,
    }
  }
}

impl<Id: Copy> Default for Error<Id> {
  fn default() -> Self {
    Self::new()
  }
}
