#[macro_export]
macro_rules! luaR_checkoffsetinbounds {
  ($inst:expr, $offset:expr) => {
    (i32::from($offset) >= 0) && (i32::from($offset) < (*(*$inst).lclass).numberofallmembers)
  };
}

pub use luaR_checkoffsetinbounds;
