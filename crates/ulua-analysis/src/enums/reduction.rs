#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Reduction {
  // The type function is either known to be reducible or the determination is blocked.
  MaybeOk,
  // The type function is known to be irreducible, but maybe not be erroneous, e.g. when it's over generics or free types.
  Irreducible,
  // The type function is known to be irreducible, and is definitely erroneous.
  Erroneous,
}
