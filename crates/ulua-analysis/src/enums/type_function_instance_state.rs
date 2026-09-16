#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum TypeFunctionInstanceState {
  /// Indicates that further reduction might be possible.
  #[default]
  Unsolved,

  /// Further reduction is not possible because one of the parameters is generic.
  Solved,

  /// Further reduction is not possible because the application is undefined.
  /// This always indicates an error in the code.
  ///
  /// eg add<nil, nil>
  Stuck,
}
