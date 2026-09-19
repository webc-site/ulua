#[macro_export]
macro_rules! luaC_needsGC {
  ($l:expr) => {
    (*(*$l).global).totalbytes >= (*(*$l).global).gc_threshold
  };
}

pub use luaC_needsGC;
