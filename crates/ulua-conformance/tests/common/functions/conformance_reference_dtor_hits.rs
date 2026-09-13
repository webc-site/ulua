use std::sync::atomic::AtomicI32;

pub static CONFORMANCE_REFERENCE_DTOR_HITS: AtomicI32 = AtomicI32::new(0);
