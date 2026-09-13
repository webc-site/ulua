use alloc::{boxed::Box, vec::Vec};

/// 稳定的堆节点存储容器，保证节点在扩容时内存地址不变（对标 C++ std::vector<std::unique_ptr<T>>）
#[derive(Debug, Clone)]
pub struct PinnedStorage<T> {
  nodes: Vec<Box<T>>,
}

impl<T> Default for PinnedStorage<T> {
  fn default() -> Self {
    Self { nodes: Vec::new() }
  }
}

impl<T> PinnedStorage<T> {
  pub fn new() -> Self {
    Self { nodes: Vec::new() }
  }

  pub fn push(&mut self, value: T) -> *mut T {
    let mut boxed = Box::new(value);
    let ptr = boxed.as_mut() as *mut T;
    self.nodes.push(boxed);
    ptr
  }

  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }
}
