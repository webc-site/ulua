use core::sync::atomic::AtomicI32;

pub static USERDATA_API_DTOR_HITS: AtomicI32 = AtomicI32::new(0);
