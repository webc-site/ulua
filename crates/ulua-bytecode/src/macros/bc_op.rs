#[macro_export]
macro_rules! BC_OP {
  ($name:ident, $idx:expr) => {
    pub(crate) const $name: u32 = $idx;

    pub(crate) fn $name(&self) -> $crate::records::bc_op::BcOp {
      self.getBcOp($idx)
    }

    paste::paste! {
        pub(crate) fn [<set $name>](&mut self, value: $crate::records::bc_op::BcOp) {
            self.setBcOp($idx, value);
        }
    }
  };
}

pub use BC_OP;
