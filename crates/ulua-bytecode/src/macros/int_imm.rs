#[macro_export]
macro_rules! INT_IMM {
  ($name:ident, $idx:expr) => {
    pub(crate) const $name: u32 = $idx;

    pub(crate) fn $name(&self) -> i32 {
      self.intImmInput($idx)
    }

    paste::paste! {
        pub(crate) fn [<set $name>](&mut self, value: i32) {
            self.setImmInput($idx, value);
        }
    }
  };
}

pub use INT_IMM;
