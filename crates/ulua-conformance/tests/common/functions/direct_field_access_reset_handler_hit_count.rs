use core::sync::atomic::Ordering;

use crate::common::records::direct_field_access_handler_hit_count::DIRECT_FIELD_ACCESS_HANDLER_HIT_COUNT;

pub fn direct_field_access_reset_handler_hit_count() {
  DIRECT_FIELD_ACCESS_HANDLER_HIT_COUNT.store(0, Ordering::SeqCst);
}
