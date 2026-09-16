#[macro_export]
macro_rules! VM_CONST {
  ($name:ident, $idx:expr) => {
    pub(crate) const $name: u32 = $idx;

    pub(crate) fn $name(
      &self,
    ) -> $crate::records::bc_ref::BcRef<$crate::records::vm_const::VmConst> {
      self.getVmConst($idx)
    }

    paste::paste! {
        pub(crate) fn [<set $name>](&mut self, cid: u32) {
            self.setVmConst($idx, cid);
        }
    }
  };
}

pub use VM_CONST;
