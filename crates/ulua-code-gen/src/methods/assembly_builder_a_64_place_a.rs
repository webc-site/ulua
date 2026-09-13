use crate::{
  enums::address_kind_a_64::AddressKindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn place_a(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src: AddressA64,
    opsize: u16,
    sizelog: i32,
  ) {
    if self.log_text {
      self.log_c_char_register_a_64_address_a_64(name, dst, src);
    }

    match src.kind {
      AddressKindA64::Reg => {
        self.place(
          dst.index() as u32
            | ((src.base.index() as u32) << 5)
            | (0b01_1010 << 10)
            | ((src.offset.index() as u32) << 16)
            | (1 << 21)
            | ((opsize as u32) << 22),
        );
      }
      AddressKindA64::Imm => {
        let data = src.data as u32;
        let shift_mask = (1 << sizelog) - 1;
        if (data >> sizelog as u32) < 1024 && (data & shift_mask) == 0 {
          self.place(
            dst.index() as u32
              | ((src.base.index() as u32) << 5)
              | ((data >> sizelog as u32) << 10)
              | ((opsize as u32) << 22)
              | (1 << 24),
          );
        } else if src.data >= -256 && src.data <= 255 {
          self.place(
            dst.index() as u32
              | ((src.base.index() as u32) << 5)
              | (((src.data as u32) & ((1 << 9) - 1)) << 12)
              | ((opsize as u32) << 22),
          );
        } else {
          self.overflowed = true;

          // Original: CODEGEN_ASSERT(!"Unable to encode large immediate offset")
          panic!("Unable to encode large immediate offset");
        }
      }
      AddressKindA64::Pre => {
        // Original code used CODEGEN_ASSERT with a pointer-based handler; avoid it here.
        assert!(src.data >= -256 && src.data <= 255);
        self.place(
          dst.index() as u32
            | ((src.base.index() as u32) << 5)
            | (0b11 << 10)
            | (((src.data as u32) & ((1 << 9) - 1)) << 12)
            | ((opsize as u32) << 22),
        );
      }
      AddressKindA64::Post => {
        // Original code used CODEGEN_ASSERT with a pointer-based handler; avoid it here.
        assert!(src.data >= -256 && src.data <= 255);
        self.place(
          dst.index() as u32
            | ((src.base.index() as u32) << 5)
            | (0b01 << 10)
            | (((src.data as u32) & ((1 << 9) - 1)) << 12)
            | ((opsize as u32) << 22),
        );
      }
    }

    self.commit();
  }
}
