extern crate alloc;

use alloc::vec::Vec;
use std::sync::Arc;

use crate::records::{event::Event, global_context::GlobalContext};

/// C++ `ThreadContext` 不可拷贝：构造即注册（`createThread`）、析构即注销
/// （`releaseThread`）。克隆体会共享同一 `thread_id`，其 Drop 将错销注册
/// 记录，故不派生 `Clone`。
#[derive(Debug)]
pub struct ThreadContext {
  pub(crate) global_context: Arc<GlobalContext>,
  pub(crate) thread_id: u32,
  pub(crate) events: Vec<Event>,
  pub(crate) data: Vec<u8>,
}

impl ThreadContext {
  pub(crate) const K_EVENT_FLUSH_LIMIT: usize = 8192;
}
