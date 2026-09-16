#[macro_export]
macro_rules! api_update_top {
  ($l:expr, $p:expr) => {{
    let l_ptr = $l;
    let p_val = $p;
    $crate::macros::api_check::api_check!(
      l_ptr,
      p_val >= (*l_ptr).base && p_val <= (*(*l_ptr).ci).top
    );
    (*l_ptr).top = p_val;
  }};
}

pub use api_update_top;
