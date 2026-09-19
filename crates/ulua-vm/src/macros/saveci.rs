//! Source: `VM/src/ldo.h` — #define saveci(l, p) ((char*)(p) - (char*)l->base_ci)
//! (hand-fixed: generated body dereferenced fields off a raw pointer without (* ))

#[macro_export]
macro_rules! saveci {
  ($l:expr, $p:expr) => {
    (($p as *const u8).offset_from((*$l).base_ci as *const u8)) as isize
  };
}

pub use saveci;
