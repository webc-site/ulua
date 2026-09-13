use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn nop(&mut self, mut length: u32) {
    while length != 0 {
      let step = if length > 9 { 9 } else { length };
      length -= step;

      match step {
        1 => {
          if self.log_text {
            self.log_append(format_args!(" nop\n"));
          }
          self.place(0x90);
        }
        2 => {
          if self.log_text {
            self.log_append(format_args!(" xchg        ax, ax ; {}-byte nop\n", step));
          }
          self.place(0x66);
          self.place(0x90);
        }
        3 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x00);
        }
        4 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x40);
          self.place(0x00);
        }
        5 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax+rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x44);
          self.place(0x00);
          self.place(0x00);
        }
        6 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         word ptr[rax+rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x66);
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x44);
          self.place(0x00);
          self.place(0x00);
        }
        7 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x80);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
        }
        8 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax+rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x84);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
        }
        9 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         word ptr[rax+rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x66);
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x84);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
        }
        _ => {}
      }

      self.commit();
    }
  }
}
