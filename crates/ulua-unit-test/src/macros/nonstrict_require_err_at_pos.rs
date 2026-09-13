#[macro_export]
macro_rules! NONSTRICT_REQUIRE_ERR_AT_POS {
  ($pos:expr, $result:expr, $idx:ident) => {
    let pos_ = $pos;
    let mut found_err = false;
    let mut index = 0;
    for err in &$result.errors {
      if err.location.begin == pos_ {
        found_err = true;
        break;
      }
      index += 1;
    }
    assert!(found_err, "Expected error at {:?}", pos_);
    $idx = index;
  };
}

pub use NONSTRICT_REQUIRE_ERR_AT_POS;
