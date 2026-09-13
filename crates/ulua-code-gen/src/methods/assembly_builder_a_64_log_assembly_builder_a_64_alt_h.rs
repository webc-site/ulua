use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn log_c_char_register_a_64_f64(&mut self, opcode: &str, dst: RegisterA64, src: f64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_register_a_64(dst);
    self.text.push(',');
    // C++ uses "#%.17g". Rust has no %g; Display ({}) emits the shortest
    // round-tripping decimal, the idiomatic equivalent (e.g. 0.25 -> "0.25").
    self.log_append(format_args!("#{}", src));
    self.text.push('\n');
  }
}
