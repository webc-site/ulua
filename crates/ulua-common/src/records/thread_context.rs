extern crate alloc;

use alloc::vec::Vec;
use std::sync::Arc;

use crate::records::{event::Event, global_context::GlobalContext};

#[derive(Debug, Clone)]
pub struct ThreadContext {
  pub(crate) global_context: Arc<GlobalContext>,
  pub(crate) thread_id: u32,
  pub(crate) events: Vec<Event>,
  pub(crate) data: Vec<u8>,
}

impl ThreadContext {
  pub(crate) const K_EVENT_FLUSH_LIMIT: usize = 8192;
}
