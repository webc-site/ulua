use crate::{
  enums::alignment_data_x_64::AlignmentDataX64, records::assembly_builder_x_64::AssemblyBuilderX64,
};

impl AssemblyBuilderX64 {
  pub fn align(&mut self, alignment: u32, data: AlignmentDataX64) {
    if !((alignment & (alignment - 1)) == 0) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = self.get_code_size();
    let pad = ((size + alignment - 1) & !(alignment - 1)) - size;

    match data {
      AlignmentDataX64::Nop => {
        if self.log_text {
          self.log_append(format_args!("; align {}\n", alignment));
        }

        self.nop(pad);
      }
      AlignmentDataX64::Int3 => {
        if self.log_text {
          self.log_append(format_args!("; align {} using int3\n", alignment));
        }

        while (self.code_pos as usize).wrapping_add(pad as usize) > self.code_end as usize {
          self.extend();
        }

        for _ in 0..pad {
          self.place(0xcc);
        }

        self.commit();
      }
      AlignmentDataX64::Ud2 => {
        if self.log_text {
          self.log_append(format_args!("; align {} using ud2\n", alignment));
        }

        while (self.code_pos as usize).wrapping_add(pad as usize) > self.code_end as usize {
          self.extend();
        }

        let mut i: u32 = 0;

        while i + 1 < pad {
          self.place(0x0f);
          self.place(0x0b);
          i += 2;
        }

        if i < pad {
          self.place(0xcc);
        }

        self.commit();
      }
    }
  }
}
