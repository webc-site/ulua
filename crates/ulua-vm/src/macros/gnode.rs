#[macro_export]
macro_rules! gnode {
  ($t:expr, $i:expr) => {
    (*$t).node.add($i as usize)
  };
}

pub use gnode;
