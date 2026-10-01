#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FragmentAutocompleteStatus {
  Success,
  FragmentTypeCheckFail,
  InternalIce,
}
