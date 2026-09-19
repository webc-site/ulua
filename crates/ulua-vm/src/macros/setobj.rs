#[macro_export]
macro_rules! setobj {
  ($l:expr, $obj1:expr, $obj2:expr) => {{
    let o2: *const $crate::type_aliases::t_value::TValue = $obj2;
    let o1: *mut $crate::type_aliases::t_value::TValue = $obj1;
    *o1 = *o2;
    $crate::macros::checkliveness::checkliveness!((*$l).global, o1);
  }};
}

pub use setobj;
