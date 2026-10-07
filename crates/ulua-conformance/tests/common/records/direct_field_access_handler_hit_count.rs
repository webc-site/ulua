use core::sync::atomic::AtomicI32;
use std::sync::Mutex;

pub static DIRECT_FIELD_ACCESS_HANDLER_HIT_COUNT: AtomicI32 = AtomicI32::new(0);
pub static DIRECT_FIELD_ACCESS_TEST_MUTEX: Mutex<()> = Mutex::new(());
