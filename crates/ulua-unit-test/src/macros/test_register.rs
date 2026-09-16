#[macro_export]
macro_rules! TEST_REGISTER {
  ($der:ident, $reg:ident, $run:ident) => {
    fn $run() {
      let mut fix = $der::default();
      fix.test();
    }

    fn $reg() {
      static mut REGISTERED: bool = false;
      unsafe {
        if !REGISTERED {
          $crate::addTestCallback($run);
          REGISTERED = true;
        }
      }
    }
  };
}

pub use TEST_REGISTER;
