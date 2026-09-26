#[macro_export]
macro_rules! twoto {
  ($x:expr) => {
    (1 << ($x)) as i32
  };
}

pub use twoto;
