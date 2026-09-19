use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn log_operand_x_64(&mut self, op: OperandX64) {
    match op.cat {
      CategoryX64::Reg => {
        let reg_name = self.get_register_name(op.base);
        self.log_append(format_args!("{}", reg_name));
      }
      CategoryX64::Mem => {
        if op.base == RegisterX64::RIP {
          if op.mem_size != SizeX64::None {
            self.log_append(format_args!("{} ptr ", self.get_size_name(op.mem_size)));
          }
          // C++ `logAppend("[.start%+d]", op.imm)` — `%+d` always shows a sign.
          self.log_append(format_args!("[.start{:+}]", op.imm));
          return;
        }

        if op.mem_size != SizeX64::None {
          self.log_append(format_args!("{} ptr ", self.get_size_name(op.mem_size)));
        }

        self.log_append(format_args!("["));

        if op.base != RegisterX64::NOREG {
          let reg_name = self.get_register_name(op.base);
          self.log_append(format_args!("{}", reg_name));
        }

        if op.index != RegisterX64::NOREG {
          let index_name = self.get_register_name(op.index);
          self.log_append(format_args!(
            "{}{}",
            if op.base != RegisterX64::NOREG {
              "+"
            } else {
              ""
            },
            index_name
          ));
        }

        if op.scale != 1 {
          self.log_append(format_args!("*{}", op.scale));
        }

        if op.imm != 0 {
          if op.imm >= 0 && op.imm <= 9 {
            self.log_append(format_args!("+{}", op.imm));
          } else if op.imm > 0 {
            self.log_append(format_args!("+0{:X}h", op.imm));
          } else {
            self.log_append(format_args!("-0{:X}h", -op.imm));
          }
        }

        self.text.push(']');
      }
      CategoryX64::Imm => {
        if op.imm >= 0 && op.imm <= 9 {
          self.log_append(format_args!("{}", op.imm));
        } else {
          self.log_append(format_args!("{:X}h", op.imm));
        }
      }
    }
  }
}
