use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn extend(&mut self) {
    let count = self.get_code_size();

    let new_size = self.code.len().wrapping_mul(2);
    self.code.resize(new_size, 0);

    let data_ptr = self.code.as_mut_ptr();
    self.code_pos = unsafe { data_ptr.add(count as usize) };
    self.code_end = unsafe { data_ptr.add(self.code.len()) };
  }
}
