use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

/// 单条 x64 指令的最大字节数，对应 cpp/CodeGen/src/AssemblyBuilderX64.cpp:68
/// `const unsigned kMaxInstructionLength = 16;`
const K_MAX_INSTRUCTION_LENGTH: usize = 16;

impl AssemblyBuilderX64 {
  pub fn commit(&mut self) {
    // CODEGEN_ASSERT(codePos <= codeEnd);
    self.instruction_count = self.instruction_count.wrapping_add(1);

    let code_pos = self.code_pos as usize;
    let code_end = self.code_end as usize;

    if (code_end.wrapping_sub(code_pos)) < K_MAX_INSTRUCTION_LENGTH {
      self.extend();
    }
  }

  pub fn extend(&mut self) {
    let count = self.get_code_size();

    let new_size = self.code.len() * 2;
    self.code.resize(new_size, 0);

    let data_ptr = self.code.as_mut_ptr();
    self.code_pos = unsafe { data_ptr.add(count as usize) };
    self.code_end = unsafe { data_ptr.add(self.code.len()) };
  }
}
