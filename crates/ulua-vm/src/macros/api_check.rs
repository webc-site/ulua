#[macro_export]
macro_rules! api_check {
  ($l:expr, $e:expr) => {
    ulua_common::LUAU_ASSERT!($e);
  };
}

pub use api_check;
