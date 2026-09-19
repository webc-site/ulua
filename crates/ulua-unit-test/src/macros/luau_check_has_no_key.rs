#[macro_export]
macro_rules! LUAU_CHECK_HAS_NO_KEY {
  ($map:expr, $key:expr) => {
    let _m = &$map;
    let _k = &$key;
    let count = _m.find(_k).is_some();
    $crate::CHECK_MESSAGE!(!count, format_args!("Map should not have key \"{:?}\"", _k));
    if count {
      $crate::MESSAGE!(format_args!("Keys: (count {})", _m.len()));
      for (k, _v) in _m.iter() {
        $crate::MESSAGE!(format_args!("\tkey: {:?}", k));
      }
    }
  };
}

pub use LUAU_CHECK_HAS_NO_KEY;
