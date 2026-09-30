//! `frontend_cancellation_token` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::sync::atomic::Ordering;

use crate::records::frontend_cancellation_token::FrontendCancellationToken;

impl FrontendCancellationToken {
  pub fn requested(&self) -> bool {
    self.cancelled.load(Ordering::Relaxed)
  }
}
